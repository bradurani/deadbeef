// use display::*;
use eval::Value;
use game::*;
use playout::playout;
use rand::rngs::SmallRng;
use repetition_detector::RepetitionDetector;
use settings::*;
use shakmaty::*;
use stats::*;
use std::f32;
use std::ops::Not;
use utils::*;

const MAX_VALUE: f32 = 900.;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NodeState {
    LeafNode,
    FullyExpanded,
    Expandable,
}

#[derive(Debug, Clone, PartialEq)]
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
    pub nn: f32,                 //new qs computed during this search
    pub nq: f32,                 //new qs computed during this search
    pub sn: f32,                 // saved n from previous searches
    pub sq: f32,                 // saved q from previous searchs. Used in UCT1, but not merged
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
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
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
            repetition_detector: RepetitionDetector::create_with_starting(game.board()),
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
            max_score: None,
            min_score: None,
        }
    }

    pub fn starting() -> TreeNode {
        TreeNode {
            outcome: None,
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 0.5,
            value: Some(Board::default().value()), //The starting position is not necessarily 0, so we calculate it
            repetition_detector: RepetitionDetector::starting(),
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
            max_score: None,
            min_score: None,
        }
    }

    pub fn clone_empty(&self) -> TreeNode {
        TreeNode {
            outcome: self.outcome,
            action: self.action,
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            value: self.value,
            repetition_detector: self.repetition_detector.clone(),
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
            max_score: None,
            min_score: None,
        }
    }

    pub fn clone_childless(&self) -> TreeNode {
        TreeNode {
            outcome: self.outcome,
            action: self.action,
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            value: self.value,
            repetition_detector: self.repetition_detector.clone(),
            nn: 0.,
            nq: 0.,
            sn: self.sn,
            sq: self.sq,
            max_score: None,
            min_score: None,
        }
    }

    // saved ns from previous searches plus ns found in this search
    // used for UT
    pub fn total_n(&self) -> f32 {
        self.sn + self.nn
    }

    pub fn total_q(&self) -> f32 {
        self.sq + self.nq
    }

    pub fn score(&self) -> f32 {
        match self.outcome {
            Some(Outcome::Decisive { winner }) => match winner {
                Color::White => f32::INFINITY,
                Color::Black => f32::NEG_INFINITY,
            },
            Some(Outcome::Draw) => 0.,
            _ => self.turn.not().coefficient() * self.sn,
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
        game.is_game_over() || game.halfmove_clock() >= MAX_HALFMOVE_CLOCK
    }

    pub fn winner(&self) -> Option<Color> {
        self.outcome.and_then(|o| o.winner())
    }

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, settings: &Settings) -> &mut TreeNode {
        // println!("\n--");
        // println!("best_child for: {}", self);
        if cfg!(debug_assertions) {
            if !self.children.iter().any(|c| c.state != NodeState::LeafNode) {
                println!("found no best children \n{}", self);
            }
        }

        let mut best_weight: f32 = f32::NEG_INFINITY;
        let mut best_child: Option<&mut TreeNode> = None;
        let self_total_n = self.total_n();
        //TODO try alpha zerp version, MCTS-Solver version and Wikipedia weighted version (are they
        //the same) can eval be used as any of the factors
        for child in &mut self.children {
            // println!("child: {}", child);
            if child.state == NodeState::LeafNode {
                continue;
            }
            let mut weight = (self.turn.coefficient() * child.total_q()) / child.total_n()
                + settings.c * (self_total_n.ln() / child.total_n()).sqrt();
            // println!("raw weight {}", weight);
            // weight += 2. * (child.normalized_color_relative_value() as f32 / child.total_n());
            weight += child.normalized_color_relative_value() * 5.;
            // println!("weighted weight {}", weight);
            // println!("value {}", value);
            //TODO what is this 2. ?????
            // println!("child: {:?} total: {}", child, child.total_n());
            // println!("value: {}", value);
            if weight > best_weight {
                best_weight = weight;
                best_child = Some(child);
            }
        }
        let found_best_child = best_child.unwrap();
        found_best_child
    }

    /// Add a child to the current node with an previously unexplored action.
    pub fn expand(
        &mut self,
        game: &mut Chess,
        candidate_actions: Vec<Move>,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
    ) -> &mut TreeNode {
        // println!("Candidate Action: {:?}", &candidate_actions);

        let action = *choose_random(rng, &candidate_actions);
        game.make_move(&action);
        let new_rep = self.repetition_detector.clone();
        let new_node = TreeNode::new(
            Some(action),
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
            child_actions.push(child.action.expect("Child node without action"));
        }

        // Find untried actions
        let mut candidate_actions: Vec<Move> = Vec::new();
        for action in &allowed_actions {
            if !child_actions.contains(action) {
                candidate_actions.push(*action);
            }
        }
        candidate_actions
    }

    fn set_outcome_based_on_child(
        &mut self,
        child_outcome: Option<Outcome>,
        _game: &mut Chess,
        thread_run_stats: &mut RunStats,
    ) {
        match child_outcome {
            // if parent has winning move
            Some(Outcome::Decisive { winner }) if winner == self.turn.not() => {
                // it's opponents turn, so checking if parent found a winner
                if self
                    .children
                    .iter()
                    .all(|c| c.winner().map_or(false, |w| w == self.turn.not()))
                {
                    self.outcome = Some(Outcome::Decisive {
                        winner: self.turn.not(),
                    });
                    self.state = NodeState::LeafNode;
                    thread_run_stats.leaf_nodes += 1;
                }
            }
            Some(Outcome::Decisive { winner }) if winner == self.turn => {
                // parent has a winning move so it's a win
                self.outcome = Some(Outcome::Decisive { winner: self.turn });
                self.state = NodeState::LeafNode;
                thread_run_stats.leaf_nodes += 1;
            }
            Some(Outcome::Draw) => {
                // self.min_score = Some(0);
                if self.children.iter().all(|c| c.is_draw()) {
                    // check for a very rare case where all grandchildren are draw by 50 move rule
                    println!("ALL GRANDCHILDREN DRAW");
                    self.max_score = Some(0);
                    self.state = NodeState::LeafNode;
                    self.outcome = Some(Outcome::Draw);
                }
            }
            _ => {}
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
        debug_assert!(game.halfmove_clock() <= MAX_HALFMOVE_CLOCK);
        // println!("{}", self);
        let delta = match self.state {
            NodeState::FullyExpanded => {
                let (delta, child_outcome) = {
                    let child = self.best_child(settings);
                    let mut child_game = game.clone(); //TODO don't clone if the move is reversible
                    child_game.make_move(&child.action.unwrap());
                    let delta = child.iteration(&mut child_game, rng, thread_run_stats, settings);
                    (delta, child.outcome)
                };
                self.set_outcome_based_on_child(child_outcome, game, thread_run_stats);
                delta
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                let candidate_actions = self.candidate_actions(allowed_actions);
                if candidate_actions.len() == 0 {
                    self.state = NodeState::FullyExpanded;
                    return self.iteration(game, rng, thread_run_stats, settings);
                }
                let last_child_expansion = candidate_actions.len() == 1;

                //advances game to position after action
                let (delta, outcome, min_score, node_state) = {
                    let mut child = self.expand(game, candidate_actions, rng, thread_run_stats);
                    if game.halfmove_clock() == MAX_HALFMOVE_CLOCK
                        || child.repetition_detector.record_and_check(game.board())
                        || game.is_stalemate()
                        || game.is_insufficient_material()
                    {
                        child.state = NodeState::LeafNode;
                        thread_run_stats.leaf_nodes += 1;
                        child.outcome = Some(Outcome::Draw);
                        child.nn += 1.;
                        (0., None, Some(0), NodeState::Expandable)
                    } else if game.is_checkmate() {
                        // println!("FOUND OUTCOME");
                        // println!("{:?}", game.board());
                        child.state = NodeState::LeafNode;
                        child.outcome = game.outcome();
                        thread_run_stats.leaf_nodes += 1;
                        let delta = child.outcome.unwrap().reward();
                        child.nn += 1.;
                        child.nq += delta;
                        (delta, child.outcome, None, NodeState::LeafNode)
                    } else {
                        let played_game = playout(rng, game, thread_run_stats);
                        let delta = played_game.outcome().map_or(0., |o| o.reward());
                        child.nn += 1.;
                        let delta = (delta + 5. * child.normalized_value()).max(-1.).min(1.);
                        child.nq += delta;
                        (delta, None, None, NodeState::Expandable)
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
                    // println!("LAST CHILD EXPANSION");
                    // println!("{:?}", game.board());
                    // if all children are draw, we're a draw
                    if self.children.iter().all(|c| c.is_draw()) {
                        self.outcome = Some(Outcome::Draw);
                        self.state = NodeState::LeafNode;
                    }
                }
                delta
            }
            _ => {
                panic!("IMPOSSIBLE ITERATION");
            }
        };
        self.nn += 1.;
        self.nq += delta;
        delta
    }

    // fn all_children_are_winning(&self, game: &Chess) -> bool {
    //     println!("all_children \n{:?}", game.board());
    //     self.children.iter().all(|child| {
    //         println!("child: {:?}", child.action);
    //         println!("child outcome: {:?}", child.outcome);
    //         match child.outcome {
    //             Some(Outcome::Decisive { winner }) if winner == self.turn => {
    //                 println!("found child mate");
    //                 println!("{:?}", child.action);
    //                 self.max_score = Some(0);
    //                 true
    //             }
    //             Some(_) => false, // stalemate or win
    //             _ => false,       // None => {
    //                                //     let mut child_game = game.clone();
    //                                //     child_game.make_move(&child.action.unwrap());
    //                                //     // println!("checking {:?}", child_game.board());
    //                                //     let allowed_actions = child_game.allowed_actions();
    //                                //     allowed_actions.iter().any(|aa| {
    //                                //         //TODO don't clone if the move is reversible
    //                                //         let mut grandchild_game = child_game.clone();
    //                                //         grandchild_game.make_move(aa);
    //                                //         // println!("IS THIS A CHECKMATE? {:?}", grandchild_game.board());
    //                                //         // if grandchild_game.is_checkmate() {
    //                                //         //     // println!("{}", "found grandchild mate");
    //                                //         // }
    //                                //         grandchild_game.is_checkmate()
    //                                //     })
    //                                // }
    //         }
    //     })
    // }
}

#[derive(Debug)]
pub struct MCTS {
    pub iterations_per_ms: f32,
}

impl MCTS {
    pub fn new(settings: &Settings) -> MCTS {
        MCTS {
            iterations_per_ms: settings.starting_iterations_per_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repetition_detector::*;
    use setup::*;

    #[test]
    fn test_iteration_mate_in_1() {
        let mut stats: RunStats = Default::default();
        let (node, _score) =
            test_iteration_all_children_with_stats("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", &mut stats);
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
            repetition_detector: RepetitionDetector::new(),
            max_score: None,
            min_score: None,
            nn: 1.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
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
        let mut game = parse_fen("8/8/8/8/8/p2k4/r7/3K4 b - - 0 1");
        let mut node = TreeNode {
            outcome: None,
            action: None,
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::Black,
            move_num: 12.,
            value: Some(6), //TODO make value not an option
            repetition_detector: RepetitionDetector::new(),
            max_score: None,
            min_score: None,
            nn: 1.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 2; // should NOT expand the winning Ra1#
        let delta = node.iteration(&mut game, &mut seeded_rng(seed), &mut stats, &settings);
        assert_eq!(-1., delta);
        assert_eq!(None, node.outcome);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::Expandable, node.state);
    }

    #[test]
    fn test_sets_min_score_if_child_is_draw() {
        let mut stats: RunStats = Default::default();
        let mut game = parse_fen("8/2kr4/8/8/8/3pK3/3Q4/8 b - - 0 1");
        let mut repetition_detector = RepetitionDetector::new();
        let drawing_position = parse_fen("8/2k1r3/8/8/8/3pK3/3Q4/8 w - - 0 1");
        repetition_detector.record_and_check(drawing_position.board());
        repetition_detector.record_and_check(drawing_position.board());
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
            nn: 1.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 6;
        let delta = node.iteration(&mut game, &mut seeded_rng(seed), &mut stats, &settings);
        println!("{}", node);
        assert_eq!(0., delta); //black is behind but has an option to draw, so delta is 0
        assert_eq!(2., node.nn);
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
        let mut repetition_detector = RepetitionDetector::new();
        let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
        let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
        repetition_detector.record_and_check(drawing_position_1.board());
        repetition_detector.record_and_check(drawing_position_1.board());
        repetition_detector.record_and_check(drawing_position_2.board());
        repetition_detector.record_and_check(drawing_position_2.board());
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
            nn: 1.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
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
        println!("{}", node);
        assert_eq!(0., delta);
        assert_eq!(3., node.nn);
        assert_eq!(Some(Outcome::Draw), node.outcome);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::LeafNode, node.state);
        assert_eq!(None, node.max_score);
        assert_eq!(Some(0), node.min_score);
    }

    #[test]
    fn sets_max_score_if_opponent_can_force_draw() {
        let mut stats: RunStats = Default::default();
        let game = parse_fen("q4rk1/5p2/8/6Q1/8/8/8/6K1 b - - 3 2");
        let mut repetition_detector = RepetitionDetector::new();
        let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
        let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
        repetition_detector.record_and_check(drawing_position_1.board());
        repetition_detector.record_and_check(drawing_position_2.board());
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
            nn: 1.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        };
        let settings = Settings::lib_test_default();
        let seed = 6;
        let mut delta = 0.;
        for _i in 1..100000 {
            delta = node.iteration(
                &mut game.clone(),
                &mut seeded_rng(seed),
                &mut stats,
                &settings,
            );
        }
        println!("{}", node);
        assert_eq!(-1., delta);
        assert_eq!(3., node.nn);
        assert_eq!(None, node.outcome);
        assert_eq!(Color::Black, node.turn);
        assert_eq!(NodeState::LeafNode, node.state);
        assert_eq!(Some(0), node.max_score);
        assert_eq!(None, node.min_score);
    }

    #[test]
    fn test_iteration_mate_in_2_1_choice() {
        let mut stats: RunStats = Default::default();
        let (node, score) =
            test_iteration_all_children_with_stats("4q3/8/8/8/8/3k4/8/3K4 b - - 0 1", &mut stats);
        println!("{}", stats);
        println!("{}", node);
        assert_eq!(-1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::Black
            },
            node.outcome.unwrap()
        );
        assert!(stats.nodes_created < 150);
    }

    #[test]
    fn test_iteration_mate_in_2_2_choices() {
        let mut stats: RunStats = Default::default();
        let (node, score) = test_iteration_all_children_with_stats(
            "8/5Q2/1pkq2n1/pB2p3/4P3/1P2K3/2P5/8 b - - 1 1",
            &mut stats,
        );
        println!("{}", stats);
        println!("{}", node);
        assert_eq!(1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::White
            },
            node.outcome.unwrap()
        );
        assert!(stats.nodes_created < 60);
    }

    fn test_iteration_all_children_with_stats(
        fen_str: &'static str,
        stats: &mut RunStats,
    ) -> (TreeNode, f32) {
        let game = parse_fen(fen_str);
        let mut settings = Settings::lib_test_default();
        let mut rng = seeded_rng(settings.starting_seed);
        let mut node = TreeNode::new_root(&game, 1.);
        let mut last = 0.;
        let mut counter = 0;
        while node.outcome.is_none() {
            last = node.iteration(&mut game.clone(), &mut rng, stats, &mut settings);
            counter += 1;
            if counter > 150 {
                println!("{}", node);
                panic!("did not find outcome");
            }
        }
        println!("found {:?}", node.outcome);
        (node, last)
    }
}
