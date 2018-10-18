// use display::*;
use eval::Value;
use eval::*;
use game::*;
use playout::playout;
use rand::rngs::SmallRng;
use settings::*;
use shakmaty::*;
use stats::*;
use std::f32;
use std::ops::Not;
use utils::*;

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
    pub nn: f32,                 //new qs computed during this search
    pub nq: f32,                 //new qs computed during this search
    pub sn: f32,                 // saved n from previous searches
    pub sq: f32,                 // saved q from previous searchs. Used in UCT1, but not merged
    pub children: Vec<TreeNode>, // next steps we investigated
}

//TODO, make all contructors take a game, and never allow manual setting of value
impl TreeNode {
    pub fn new(action: Option<Move>, turn: Color, move_num: f32, value: Option<i16>) -> TreeNode {
        TreeNode {
            outcome: None,
            action: action,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: turn,
            move_num: move_num,
            value: value,
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
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
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
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
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
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
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
            nn: 0.,
            nq: 0.,
            sn: self.sn,
            sq: self.sq,
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

    pub fn is_decisive(&self) -> bool {
        match self.outcome {
            Some(Outcome::Decisive { winner: _ }) => true,
            _ => false,
        }
    }

    pub fn is_decided(&self) -> bool {
        self.outcome.is_some()
    }

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, settings: &Settings) -> &mut TreeNode {
        let mut best_weight: f32 = f32::NEG_INFINITY;
        let mut best_child: Option<&mut TreeNode> = None;
        let self_total_n = self.total_n();
        //TODO try alpha zerp version, MCTS-Solver version and Wikipedia weighted version (are they
        //the same) can eval be used as any of the factors
        // println!("\n--");
        // println!("best_child: {}", self);
        for child in &mut self.children {
            // println!("child: {}", child);
            if child.state == NodeState::LeafNode {
                continue;
            }
            let mut weight = (self.turn.coefficient() * child.total_q()) / child.total_n()
                + settings.c * (2. * self_total_n.ln() / child.total_n()).sqrt();
            // println!("raw weight {}", weight);
            weight += child.color_relative_value() as f32 / child.total_n();
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

        self.children.push(TreeNode::new(
            Some(action),
            self.turn.not(),
            self.move_num + 0.5,
            Some(game.board().value()),
        ));
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

    fn new_outcome_based_on_child(
        child_outcome: Option<Outcome>,
        turn: Color,
        children: &Vec<TreeNode>,
        game: &mut Chess,
    ) -> Option<Outcome> {
        //TODO, do we need to cache the results of this? otherwise it's calculated on every
        //traveral
        match child_outcome {
            Some(Outcome::Decisive { winner: color }) if color == turn.not() => {
                // println!("checking for child mate. Looking for grandchildren");
                // println!("{:?}", game.board());
                if TreeNode::all_children_have_winning_grandchild(children, &game) {
                    Some(Outcome::Decisive { winner: turn.not() }) //can't escape checkmate. All move are a win for opponent
                } else {
                    None
                }
            }
            Some(Outcome::Decisive { winner: color }) if color == turn => {
                Some(Outcome::Decisive { winner: turn })
            }
            Some(Outcome::Draw) => {
                if children.iter().all(|c| c.outcome.is_some()) {
                    //all children are draws or checkmate for opponent, so it's a draw for us
                    debug_assert!(
                        !children
                            .iter()
                            .any(|c| c.outcome.unwrap().winner().unwrap() == turn)
                    );
                    Some(Outcome::Draw)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Recursively perform an MCTS iteration.
    ///
    /// XXX A non-recursive implementation would probably be faster.
    /// XXX But how to keep &mut pointers to all our parents while
    /// XXX we fiddle with our leaf node?
    pub fn iteration(
        &mut self,
        game: &mut Chess,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
        settings: &Settings,
    ) -> f32 {
        thread_run_stats.iterations += 1;
        // println!("{}", self);
        let mut delta = match self.state {
            // NodeState::LeafNode => {
            //     // println!("{}", game.outcome().unwrap());
            //     // game.reward()
            //     thread_run_stats.leaf_nodes += 1;
            //     self.outcome.unwrap().reward()
            // }
            NodeState::FullyExpanded => {
                let (delta, outcome) = {
                    let child = self.best_child(settings);
                    let mut child_game = game.clone(); //TODO don't clone if the move is reversible
                    child_game.make_move(&child.action.unwrap());
                    let delta = child.iteration(&mut child_game, rng, thread_run_stats, settings);
                    (delta, child.outcome)
                };

                //we've now looked at the first grandchild node, which has propogated the win
                //back up if it's a checkmate for our opponent. Now check all of them to see if
                //we can't avoid mate in N
                let outcome_based_on_children = TreeNode::new_outcome_based_on_child(
                    outcome,
                    game.turn(),
                    &self.children,
                    game,
                );
                if outcome_based_on_children.is_some() {
                    self.state = NodeState::LeafNode;
                }
                self.outcome = outcome_based_on_children;

                delta
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                //TODO cleanup
                if allowed_actions.len() == 0 || game.is_insufficient_material() {
                    //TODO or 50 move rule
                    self.state = NodeState::LeafNode;
                    self.outcome = game.outcome();
                    return self.outcome.unwrap().reward();
                }
                let candidate_actions = self.candidate_actions(allowed_actions);
                if candidate_actions.len() == 0 {
                    //if we ended up expanded as the result fo a tree merge
                    //TODO check if merging filled out all outcomes
                    //TODO fix bugs related to
                    self.outcome = self.outcome_based_on_immediate_children();
                    if self.outcome.is_some() {
                        self.state = NodeState::LeafNode;
                        return self.outcome.unwrap().reward();
                    }
                    self.state = NodeState::FullyExpanded;
                    return self.iteration(game, rng, thread_run_stats, settings);
                }

                //advances game to position after action
                let (delta, outcome) = {
                    let mut child = self.expand(game, candidate_actions, rng, thread_run_stats);
                    if game.is_game_over() {
                        let child_turn = game.turn().not();
                        // println!("FOUND CHECKMATE");
                        // println!("{:?}", game.board());
                        child.state = NodeState::LeafNode;
                        child.outcome = game.outcome();
                        let delta = child.outcome.unwrap().reward();
                        child.nn += 1.;
                        child.nq += delta;
                        thread_run_stats.leaf_nodes += 1;
                        (delta, child.outcome)
                    } else {
                        let played_game = playout(rng, game, thread_run_stats);
                        let delta = played_game.outcome().map(|o| o.reward()).unwrap_or(0.);
                        child.nn += 1.;
                        child.nq += delta;
                        (delta, None)
                    }
                };
                match outcome {
                    None => {}
                    Some(Outcome::Decisive { winner }) => {
                        // opponent can mate next move. Game is lost
                        self.state = NodeState::LeafNode;
                        self.outcome = outcome;
                    }
                    Some(Outcome::Draw) => {
                        // opponent can force a draw
                        // could we use this to prevent forced draw if we're ahead?
                    }
                }
                delta
            }
            _ => {
                println!("IMPOSSIBLE ITERATION");
                println!("{}", self);
                panic!("unknown leaf node type")
            }
        };
        self.nn += 1.;
        self.nq += delta;
        delta
    }

    fn all_children_have_winning_grandchild(children: &Vec<TreeNode>, game: &Chess) -> bool {
        children.iter().all(|child| {
            match child.outcome {
                Some(Outcome::Decisive { winner: color }) if color == game.turn().not() => {
                    // println!("found child mate");
                    // println!("{:?}", child.action);
                    true
                }
                Some(_) => false, // stalemate or win
                None => {
                    let mut child_game = game.clone();
                    child_game.make_move(&child.action.unwrap());
                    // println!("checking {:?}", child_game.board());
                    let allowed_actions = child_game.allowed_actions();
                    allowed_actions.iter().any(|aa| {
                        //TODO don't clone if the move is reversible
                        let mut grandchild_game = child_game.clone();
                        grandchild_game.make_move(aa);
                        // println!("IS THIS A CHECKMATE? {:?}", grandchild_game.board());
                        // if grandchild_game.is_checkmate() {
                        //     // println!("{}", "found grandchild mate");
                        // }
                        grandchild_game.is_checkmate()
                    })
                }
            }
        })
    }

    // If we fill up all the children, we have to check to see if they're all draws so we can
    // propogate draw upwards. Also, we could ahve filled all children with own wins during tree
    // merge, so we need to propogate that up too
    fn outcome_based_on_immediate_children(&self) -> Option<Outcome> {
        debug_assert!(self.outcome.is_none());
        let mut outcome = Some(Outcome::Decisive {
            winner: self.turn.not(),
        }); //unless I find something better, it's a win for my opponent
        for child in self.children.iter() {
            match child.outcome {
                Some(Outcome::Decisive { winner }) if winner == self.turn => {
                    panic!("found own win");
                } //if I have a winning move, I should already be set to LeafNode
                Some(Outcome::Decisive { winner }) if winner == self.turn.not() => {
                    continue;
                }
                Some(Outcome::Draw) => outcome = Some(Outcome::Draw),
                None => {
                    outcome = None;
                    break;
                }
                _ => {
                    panic!("impossible outcome");
                }
            }
        }
        outcome
    }
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
    use mcts::TreeNode;
    use settings::*;
    use setup::*;
    use shakmaty::{Color, Outcome};
    use stats::RunStats;
    use utils::*;

    #[test]
    fn iteration_mate_in_1() {
        let mut stats: RunStats = Default::default();
        let (node, score) =
            test_iteration_all_children_with_stats("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", &mut stats);
        assert_eq!(1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::White
            },
            node.outcome.unwrap()
        );
        assert!(stats.iterations < 50);
        println!("{}", stats);
    }

    #[test]
    fn iteration_mate_in_2_1_choice() {
        let mut stats: RunStats = Default::default();
        let (node, score) =
            test_iteration_all_children_with_stats("4q3/8/8/8/8/3k4/8/3K4 b - - 0 1", &mut stats);
        assert_eq!(-1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::Black
            },
            node.outcome.unwrap()
        );
        assert!(stats.iterations < 200);
        println!("{}", stats);
        println!("{}", node);
    }

    #[test]
    fn iteration_mate_in_2_2_choices() {
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
        assert!(stats.iterations < 60);
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
            if counter > 100000 {
                println!("{}", node);
                panic!("did not find checkmate");
            }
        }
        println!("found {:?}", node.outcome);
        (node, last)
    }
}
