// use display::*;
use eval::Value;
use game::*;
// use playout::playout;
use rand::rngs::SmallRng;
use repetition_detector::RepetitionDetector;
use settings::*;
use shakmaty::*;
use stats::*;
use std::f32;
use std::ops::Not;
use uct::*;
use utils::*;

const MAX_VALUE: f32 = 900.;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NodeState {
    LeafNode,
    FullyExpanded,
    Expandable,
}

impl Default for NodeState {
    fn default() -> NodeState {
        NodeState::Expandable
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeNode {
    pub outcome: Option<Outcome>,
    pub action: Option<Move>, // how did we get here
    pub value: Option<i16>,
    pub state: NodeState, // is this a leaf node? fully expanded?
    //TODO don't need turn
    pub turn: Color, //which player made this move
    //TODO don't need move number
    pub move_num: f32,
    pub repetition_detector: RepetitionDetector,
    pub n: f32,                  //new qs computed during this search
    pub q: f32,                  //new qs computed during this search
    pub children: Vec<TreeNode>, // next steps we investigated
    pub max_score: Option<u16>,
    pub min_score: Option<u16>,
}

//TODO, make all contructors take a game, and never allow manual setting of value
impl TreeNode {
    pub fn new(
        action: Option<Move>,
        turn: Color,
        move_num: f32,
        value: Option<i16>,
        rd: RepetitionDetector,
    ) -> TreeNode {
        TreeNode {
            outcome: None,
            action: action,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: turn,
            move_num: move_num,
            value: value,
            repetition_detector: rd,
            max_score: None,
            min_score: None,
            n: 0.,
            q: 0.,
        }
    }

    pub fn new_root(game: &Chess, move_num: f32) -> TreeNode {
        TreeNode {
            outcome: None,
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: game.turn(),  // So we switch to White for move 1
            move_num: move_num, //So we increment to 1 for move 1
            value: Some(game.board().value()),
            repetition_detector: RepetitionDetector::new(game),
            n: 0.,
            q: 0.,
            max_score: None,
            min_score: None,
        }
    }

    // pub fn starting() -> TreeNode {
    //     TreeNode {
    //         outcome: None,
    //         action: None,
    //         children: Vec::new(),
    //         state: NodeState::Expandable,
    //         turn: Color::White,
    //         move_num: 0.5,
    //         value: Some(Board::default().value()), //The starting position is not necessarily 0, so we calculate it
    //         repetition_detector: RepetitionDetector::default(),
    //         n: 0.,
    //         q: 0.,
    //         max_score: None,
    //         min_score: None,
    //     }
    // }

    pub fn clone_childless(&self) -> TreeNode {
        TreeNode {
            outcome: self.outcome,
            action: self.action.clone(),
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            value: self.value,
            repetition_detector: self.repetition_detector.clone(),
            n: self.n,
            q: self.q,
            max_score: self.max_score,
            min_score: self.min_score,
        }
    }

    pub fn score(&self) -> f32 {
        match self.outcome {
            Some(Outcome::Decisive { winner }) => match winner {
                Color::White => f32::INFINITY,
                Color::Black => f32::NEG_INFINITY,
            },
            Some(Outcome::Draw) => 0.,
            _ => (self.turn.not().coefficient() * self.n),
        }
    }

    pub fn adjusted_q(&self) -> f32 {
        let mut adjusted_q = self.turn.not().coefficient() * self.q;
        adjusted_q = match self.min_score {
            Some(min) => adjusted_q.min(min as f32), // opponent can do no worse than min so we can do no better than min
            None => adjusted_q,
        };
        match self.max_score {
            Some(max) => adjusted_q.max(max as f32),
            None => adjusted_q,
        }
    }

    pub fn color_relative_score(&self) -> f32 {
        self.score() * self.turn.not().coefficient()
    }

    pub fn color_relative_value(&self) -> f32 {
        self.value.unwrap() as f32 * self.turn.not().coefficient()
    }

    pub fn normalized_value(&self) -> f32 {
        (self.value.unwrap() as f32 / MAX_VALUE)
    }

    pub fn normalized_color_relative_value(&self) -> f32 {
        (self.value.unwrap() as f32 / MAX_VALUE) * self.turn.not().coefficient()
    }

    pub fn is_decisive(&self) -> bool {
        match self.outcome {
            Some(Outcome::Decisive { winner: _ }) => true,
            _ => false,
        }
    }

    pub fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn is_draw(&self) -> bool {
        match self.outcome {
            Some(Outcome::Draw) => true,
            _ => false,
        }
    }

    pub fn is_game_over_or_drawn(&self, game: &Chess) -> bool {
        game.is_game_over() || game.halfmoves() >= MAX_HALFMOVES
    }

    pub fn winner(&self) -> Option<Color> {
        self.outcome.and_then(|o| o.winner())
    }

    /// Add a child to the current node with an previously unexplored action.
    pub fn expand(
        &mut self,
        game: &mut Chess,
        candidate_actions: Vec<Move>,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
    ) -> &mut TreeNode {
        let action = choose_random(rng, &candidate_actions);
        game.make_move(action);
        let new_rep = self.repetition_detector.clone();
        let new_node = TreeNode::new(
            Some(action.clone()),
            self.turn.not(),
            self.move_num + 0.5,
            Some(game.board().value()),
            new_rep,
        );
        self.children.push(new_node);
        thread_run_stats.nodes_created += 1;
        self.children.last_mut().unwrap()
    }

    fn candidate_actions(&self, allowed_actions: Vec<Move>) -> Vec<Move> {
        // What are our options given the current game state?
        // could save this between calls

        // Get a list with all the actions we tried alreday
        let mut child_actions: Vec<Move> = Vec::new();
        for child in &self.children {
            child_actions.push(child.action.clone().expect("Child node without action"));
        }

        // Find untried actions
        let mut candidate_actions: Vec<Move> = Vec::new();
        for action in &allowed_actions {
            if !child_actions.contains(action) {
                candidate_actions.push(action.clone());
            }
        }
        candidate_actions
    }

    pub fn set_outcome_from_children(&mut self, stats: &mut RunStats) {
        if self
            .children
            .iter()
            .any(|c| c.outcome == Some(Outcome::Decisive { winner: self.turn }))
        {
            // one of the children is a winning move for this parent, so this node is a winner
            self.outcome = Some(Outcome::Decisive { winner: self.turn });
        } else if self.children.iter().all(|c| {
            c.outcome == Some(Outcome::Decisive {
                winner: self.turn.not(),
            })
        }) {
            // if all children can force a win for opponent, this node is win for opponent
            self.outcome = Some(Outcome::Decisive {
                winner: self.turn.not(),
            });
        } else if self.children.iter().all(|c| {
            c.outcome == Some(Outcome::Draw) || c.outcome == Some(Outcome::Decisive {
                winner: self.turn.not(),
            })
        }) {
            self.outcome = Some(Outcome::Draw);
            self.max_score = Some(0);
            self.min_score = Some(0);
        } else if self.children.iter().any(|c| c.max_score == Some(0)) {
            // one of my children allows me to force a draw, the move leading to this position is at
            // best, a draw for my opponent
            self.min_score = Some(0)
        }
        // no else because I don't belive this is mutually exclusive to the above condition
        if self.children.iter().all(|c| c.min_score == Some(0)) {
            // all of my children allow opponent to force a draw, so the move leading to this is
            // at worst a draw for my opponent
            self.max_score = Some(0)
        }
        if self.outcome.is_some() {
            self.state = NodeState::LeafNode;
            stats.leaf_nodes += 1;
        }
    }

    pub fn iteration(
        &mut self,
        game: &mut Chess,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
        settings: &Settings,
    ) -> f32 {
        thread_run_stats.iterations += 1;
        debug_assert!(game.halfmoves() <= MAX_HALFMOVES);
        let delta = match self.state {
            NodeState::FullyExpanded => {
                let (delta, child_changes_outcome) = {
                    let child = best_child(self, settings);
                    let mut child_game = game.clone(); //TODO don't clone if the move is reversible
                    child_game.make_move(&child.action.clone().unwrap());
                    let delta = child.iteration(&mut child_game, rng, thread_run_stats, settings);
                    (
                        delta,
                        child.outcome.is_some()
                            || child.min_score.is_some()
                            || child.max_score.is_some(),
                    )
                };
                if child_changes_outcome {
                    // this calc gets repeated a lot unnecesarily and can be made more efficient
                    self.set_outcome_from_children(thread_run_stats);
                }
                delta
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                let candidate_actions = self.candidate_actions(allowed_actions);
                debug_assert!(candidate_actions.len() > 0);
                let mut last_child_expansion = false;
                let mut new_state = self.state;

                if candidate_actions.len() == 1 {
                    new_state = NodeState::FullyExpanded;
                    last_child_expansion = true;
                }

                //advances game to position after action
                let (delta, outcome, min_score, node_state) = {
                    let mut child = self.expand(game, candidate_actions, rng, thread_run_stats);
                    if game.halfmoves() == MAX_HALFMOVES
                        || child.repetition_detector.record_and_check(game)
                        || game.is_stalemate()
                        || game.is_insufficient_material()
                    {
                        child.state = NodeState::LeafNode;
                        thread_run_stats.leaf_nodes += 1;
                        child.outcome = Some(Outcome::Draw);
                        child.n += 1.;
                        (0., None, Some(0), new_state)
                    } else if game.is_checkmate() {
                        child.state = NodeState::LeafNode;
                        child.outcome = game.outcome();
                        thread_run_stats.leaf_nodes += 1;
                        let delta = child.outcome.unwrap().reward();
                        child.n += 1.;
                        child.q += delta;
                        (delta, child.outcome, None, NodeState::LeafNode)
                    } else {
                        // let played_game = playout(rng, game, thread_run_stats);
                        // let delta = played_game.outcome().map_or(0., |o| o.reward());
                        child.n += 1.;
                        let delta = child.normalized_value(); //delta + child.normalized_value();
                        child.q += delta;
                        (delta, None, None, new_state)
                    }
                };
                if self.min_score == None {
                    self.min_score = min_score;
                }
                if self.outcome == None {
                    self.outcome = outcome;
                }
                self.state = node_state;
                if last_child_expansion {
                    self.set_outcome_from_children(thread_run_stats)
                }
                delta
            }
            _ => {
                panic!("IMPOSSIBLE ITERATION");
            }
        };
        self.n += 1.;
        self.q += delta;
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use display::*;
    use play;
    use repetition_detector::*;
    use setup::*;
    use shakmaty::*;

    #[test]
    fn test_iteration_mate_in_1() {
        let mut stats: RunStats = Default::default();
        let settings = Settings::lib_test_default();
        let (node, _score) =
            test_iteration_all_children("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", &mut stats, &settings);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::White
            },
            node.outcome.unwrap()
        );
        assert!(stats.iterations < 50);
        assert_eq!(NodeState::LeafNode, node.state);
        println!("{}", stats);
    }

    #[test]
    fn test_decisive_if_child_is_win() {
        let mut stats: RunStats = Default::default();
        let mut game = parse_fen("8/8/8/8/8/p2k4/r7/3K4 b - - 0 1");
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::Black,
            move_num: 12.,
            value: Some(6), //TODO make value not an option
            repetition_detector: RepetitionDetector::new(&game),
            max_score: None,
            min_score: None,
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 1; // should expand Ra1#
        let delta = node.iteration(&mut game, &mut seeded_rng(seed), &mut stats, &settings);
        assert_eq!(-1., delta);
        assert_eq!(Some(Color::Black), node.winner());
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::LeafNode, node.state);
    }

    #[test]
    fn test_delta_1_in_dominate_position() {
        let mut stats: RunStats = Default::default();
        let mut game = parse_fen("2r1q2k/4p2n/4P3/8/3NK3/1p6/2p5/8 w - - 0 1");
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::Black,
            move_num: 12.,
            value: Some(6), //TODO make value not an option
            repetition_detector: RepetitionDetector::new(&game),
            max_score: None,
            min_score: None,
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 2; // should NOT expand the winning Ra1#
        let delta = node.iteration(&mut game, &mut seeded_rng(seed), &mut stats, &settings);
        print_tree(&node, &settings);
        assert!(delta < -1.);
        assert_eq!(None, node.outcome);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::Expandable, node.state);
    }

    #[test]
    fn test_sets_min_score_if_child_is_draw() {
        let mut stats: RunStats = Default::default();
        let mut game = parse_fen("8/2kr4/8/8/8/3pK3/3Q4/8 b - - 0 1");
        let mut repetition_detector = RepetitionDetector::new(&game);
        let drawing_position = parse_fen("8/2k1r3/8/8/8/3pK3/3Q4/8 w - - 0 1");
        repetition_detector.record_and_check(&drawing_position);
        repetition_detector.record_and_check(&drawing_position);
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::Black,
            move_num: 12.,
            value: Some(-100), //TODO make value not an option
            repetition_detector: repetition_detector,
            max_score: None,
            min_score: Some(0),
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 6;
        let delta = node.iteration(&mut game, &mut seeded_rng(seed), &mut stats, &settings);
        print_tree(&node, &settings);
        assert_eq!(0., delta); //black is behind but has an option to draw, so delta is 0
        assert_eq!(2., node.n);
        assert_eq!(None, node.outcome);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::Expandable, node.state);
        assert_eq!(node.min_score, Some(0));
        assert_eq!(node.max_score, None);
    }

    #[test]
    fn test_is_draw_if_all_children_are_draws() {
        let mut stats: RunStats = Default::default();
        let game = parse_fen("q4rk1/5p2/8/6Q1/8/8/8/6K1 b - - 3 2");
        let mut repetition_detector = RepetitionDetector::new(&game);
        let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
        let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
        repetition_detector.record_and_check(&drawing_position_1);
        repetition_detector.record_and_check(&drawing_position_1);
        repetition_detector.record_and_check(&drawing_position_2);
        repetition_detector.record_and_check(&drawing_position_2);
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::Black,
            move_num: 12.,
            value: Some(-100), //TODO make value not an option
            repetition_detector: repetition_detector,
            max_score: None,
            min_score: None,
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 6;
        node.iteration(
            &mut game.clone(),
            &mut seeded_rng(seed),
            &mut stats,
            &settings,
        );
        let delta = node.iteration(
            &mut game.clone(),
            &mut seeded_rng(seed),
            &mut stats,
            &settings,
        );
        print_tree(&node, &settings);
        assert_eq!(0., delta);
        assert_eq!(3., node.n);
        assert_eq!(Some(Outcome::Draw), node.outcome);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::LeafNode, node.state);
        assert_eq!(Some(0), node.max_score);
        assert_eq!(Some(0), node.min_score);
    }

    #[test]
    fn test_sets_min_score_if_opponent_can_force_draw() {
        let mut stats: RunStats = Default::default();
        let game = parse_fen("q4rk1/5p2/8/6Q1/8/8/8/6K1 b - - 3 2");
        let mut repetition_detector = RepetitionDetector::new(&game);
        let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
        let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
        repetition_detector.record_and_check(&drawing_position_1);
        repetition_detector.record_and_check(&drawing_position_1);
        repetition_detector.record_and_check(&drawing_position_2);
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::Black,
            move_num: 12.,
            value: Some(-100), //TODO make value not an option
            repetition_detector: repetition_detector,
            max_score: None,
            min_score: None,
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 6;
        let mut delta = 0.;
        let n = 20000.;
        for _i in 0..n as u32 {
            delta = node.iteration(
                &mut game.clone(),
                &mut seeded_rng(seed),
                &mut stats,
                &settings,
            );
            if node.max_score.is_some() || node.outcome.is_some() {
                break;
            }
        }
        print_tree(&node, &settings);
        assert_eq!(None, node.outcome);
        // assert_eq!(-1., delta);
        assert_eq!(n + 1., node.n);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::FullyExpanded, node.state);
        assert_eq!(None, node.max_score);
        assert_eq!(Some(0), node.min_score);
    }

    #[test]
    fn test_outcome_is_draw_if_lose_or_draw() {
        let mut stats: RunStats = Default::default();
        let game = parse_fen("1q3k2/8/8/8/8/8/r7/6K1 w - - 1 1");
        let mut repetition_detector = RepetitionDetector::new(&game);
        let drawing_position = parse_fen("1q3k2/8/8/8/8/8/r7/5K2 b - - 2 1");
        repetition_detector.record_and_check(&drawing_position);
        repetition_detector.record_and_check(&drawing_position);
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 12.,
            value: Some(-100), //TODO make value not an option
            repetition_detector: repetition_detector,
            max_score: None,
            min_score: None,
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 1;
        let mut delta = 0.;
        let n = 17.;
        for _i in 0..n as u32 {
            delta = node.iteration(
                &mut game.clone(),
                &mut seeded_rng(seed),
                &mut stats,
                &settings,
            );
            if node.outcome.is_some() {
                break;
            }
        }
        print_tree(&node, &settings);
        assert_eq!(Some(Outcome::Draw), node.outcome);
        // assert_eq!(-1., delta);
        assert_eq!(n + 1., node.n);
        assert_eq!(Color::White, node.turn);
        assert_eq!(NodeState::LeafNode, node.state);
        assert_eq!(Some(0), node.min_score);
        assert_eq!(Some(0), node.max_score);
    }

    #[test]
    fn test_draws_intentionally_if_behind() {
        // test that if there's a draw on the next move and we're way behind, we draw,
        // even though exploration will rack up n for other nodes, so we can't just
        // rely on n to choose best move
        let mut stats: RunStats = Default::default();
        let game = parse_fen("n1nqk3/b1ppp3/1p6/p7/5P2/4P1P1/4NK2/8 w - - 0 1");
        let mut repetition_detector = RepetitionDetector::new(&game);
        let drawing_position = parse_fen("n1nqk3/b1ppp3/1p6/p7/5P2/4P1P1/4N3/5K2 b - - 0 1");
        repetition_detector.record_and_check(&drawing_position);
        repetition_detector.record_and_check(&drawing_position);
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 12.,
            value: Some(-100), //TODO make value not an option
            repetition_detector: repetition_detector,
            max_score: None,
            min_score: None,
            n: 1.,
            q: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 1;
        let mut delta = 0.;
        let n = 50;
        for _i in 0..n as u32 {
            delta = node.iteration(
                &mut game.clone(),
                &mut seeded_rng(seed),
                &mut stats,
                &settings,
            );
            if node.outcome.is_some() {
                break;
            }
        }
        print_tree(&node, &settings);
        assert_eq!(None, node.outcome);
        assert_eq!(n as f32 + 1., node.n);
        assert_eq!(Color::White, node.turn);
        assert_eq!(NodeState::FullyExpanded, node.state);
        assert_eq!(Some(0), node.min_score);
        assert_eq!(None, node.max_score);
        let best_move = play::find_best_move(node, &game, &mut stats, &settings).unwrap();
        assert_eq!(
            Move::Normal {
                role: Role::King,
                from: Square::F2,
                capture: None,
                to: Square::F1,
                promotion: None
            },
            best_move.action.unwrap()
        );
    }

    #[test]
    fn test_iteration_mate_in_2_1_choice() {
        let mut stats: RunStats = Default::default();
        let settings = Settings::lib_test_default();
        let (node, score) =
            test_iteration_all_children("4q3/8/8/8/8/3k4/8/3K4 b - - 0 1", &mut stats, &settings);
        println!("{}", stats);
        print_tree(&node, &settings);
        assert_eq!(-1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::Black
            },
            node.outcome.unwrap()
        );
        assert!(stats.nodes_created < 1000);
    }

    #[test]
    fn test_iteration_mate_in_2_2_choices() {
        let mut stats: RunStats = Default::default();
        let settings = Settings::lib_test_default();
        let (node, score) = test_iteration_all_children(
            "8/5Q2/1pkq2n1/pB2p3/4P3/1P2K3/2P5/8 b - - 1 1",
            &mut stats,
            &settings,
        );
        println!("{}", stats);
        print_tree(&node, &settings);
        assert_eq!(1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::White
            },
            node.outcome.unwrap()
        );
        assert!(stats.nodes_created < 60);
    }

    fn test_iteration_all_children(
        fen_str: &'static str,
        stats: &mut RunStats,
        settings: &Settings,
    ) -> (TreeNode, f32) {
        let game = parse_fen(fen_str);
        let mut rng = seeded_rng(settings.starting_seed);
        let mut node = TreeNode::new_root(&game, 1.);
        let mut last = 0.;
        let mut counter = 0;
        while node.outcome.is_none() {
            last = node.iteration(&mut game.clone(), &mut rng, stats, &settings);
            counter += 1;
            if counter > 1000 {
                print_tree(&node, &settings);
                panic!("did not find outcome");
            }
        }
        println!("found {:?}", node.outcome);
        (node, last)
    }
}
