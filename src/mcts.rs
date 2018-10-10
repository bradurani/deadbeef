extern crate shakmaty; //?

use shakmaty::*;
use std::cmp::{max, min};
use std::f32;
use std::fmt;
use std::i32;
use std::ops::Not;
use std::thread;
use std::thread::JoinHandle;

use std::time::Instant;

use playout::playout;
use rand::rngs::SmallRng;
use tree_merge::merge_trees;
use utils::*;

const STARTING_ITERATIONS_PER_MS: f32 = 1.;

/// A `Game` represets a game state.
///
/// It is important that the game behaves fully deterministic,
/// e.g. it has to produce the same game sequences
pub trait Game: Clone {
    /// Return a list with all allowed actions given the current game state.
    fn allowed_actions(&self) -> Vec<Move>;

    /// Change the current game state according to the given action.
    fn make_move(&mut self, action: &Move);

    /// Reward for the player when reaching the current game state.
    fn reward(&self) -> f32;
}

//////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NodeState {
    LeafNode,
    FullyExpanded,
    Expandable,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TreeNode {
    pub action: Option<Move>, // how did we get here
    pub state: NodeState,     // is this a leaf node? fully expanded?
    pub turn: Color,          //which player made this move
    pub move_num: f32,
    pub nn: f32,                 //new qs computed during this search
    pub nq: f32,                 //new qs computed during this search
    pub sn: f32,                 // saved n from previous searches
    pub sq: f32,                 // saved q from previous searchs. Used in UCT1, but not merged
    pub children: Vec<TreeNode>, // next steps we investigated
}

impl Game for Chess {
    fn allowed_actions(&self) -> Vec<Move> {
        match &self.is_game_over() {
            true => Vec::new(),
            false => {
                let mut moves = MoveList::new();
                self.legal_moves(&mut moves);
                moves.to_vec()
            }
        }
    }

    fn make_move(&mut self, action: &Move) {
        self.play_unchecked(&action);
        // self.play_safe(&action)
        // TODO add safe option for testing
    }

    fn reward(&self) -> f32 {
        let outcome = self.outcome();
        match outcome {
            Some(Outcome::Decisive {
                winner: Color::Black,
            }) => -1.0,
            Some(Outcome::Decisive {
                winner: Color::White,
            }) => 1.0,
            Some(Outcome::Draw) => 0.0,
            None => 0.0,
        }
    }
}

impl TreeNode {
    /// Create and initialize a new TreeNode
    ///
    /// Initialize q and n t to be zero; childeren list to
    /// be empty and set the node state to Expandable.
    pub fn new(action: Option<Move>, turn: Color, move_num: f32) -> TreeNode {
        TreeNode {
            action: action,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: turn,
            move_num: move_num,
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn new_root(game: &Chess, move_num: f32) -> TreeNode {
        TreeNode {
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: game.turn(),  // So we switch to White for move 1
            move_num: move_num, //So we increment to 1 for move 1
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn starting() -> TreeNode {
        TreeNode {
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 0.5,
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn clone_empty(&self) -> TreeNode {
        TreeNode {
            action: self.action,
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn clone_childless(&self) -> TreeNode {
        TreeNode {
            action: self.action,
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            nn: 0.,
            nq: 0.,
            sn: self.sn,
            sq: self.sq,
        }
    }

    // saved ns from previous searches plus ns found in this search
    // used for UCT1 to guide search efforts
    pub fn total_n(&self) -> f32 {
        self.sn + self.nn
    }

    pub fn total_q(&self) -> f32 {
        self.sq + self.nq
    }

    pub fn score(&self) -> f32 {
        self.total_q() as f32 / self.total_n() as f32
    }

    pub fn child_by_move(&self, action: &Move) -> &TreeNode {
        let next_root = self
            .children
            .iter()
            .find(|c| &c.action.unwrap() == action)
            .unwrap();
        next_root
    }

    /// Gather some statistics about this subtree
    pub fn tree_statistics(&self) -> TreeStatistics {
        let child_stats = self
            .children
            .iter()
            .map(|c| c.tree_statistics())
            .collect::<Vec<_>>();
        TreeStatistics::merge(&child_stats)
    }

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, c: f32) -> &mut TreeNode {
        let mut best_value: f32 = f32::NEG_INFINITY;
        let mut best_child: Option<&mut TreeNode> = None;
        let self_total_n = self.total_n();

        for child in &mut self.children {
            let value = (self.turn.coefficient() * child.total_q()) / child.total_n()
                + c * (2. * self_total_n.ln() / child.total_n()).sqrt();
            // println!("child: {:?} total: {}", child, child.total_n());
            // println!("value: {}", value);
            if value > best_value {
                best_value = value;
                best_child = Some(child);
            }
        }
        let found_best_child = best_child.unwrap();
        // println!("Best child for {:?}: {}", child_turn, found_best_child);
        found_best_child
    }

    /// Add a child to the current node with an previously unexplored action.
    pub fn expand(&mut self, candidate_actions: Vec<Move>, rng: &mut SmallRng) -> &mut TreeNode {
        // println!("Candidate Action: {:?}", &candidate_actions);

        let action = *choose_random(rng, &candidate_actions);

        self.children.push(TreeNode::new(
            Some(action),
            self.turn.not(),
            self.move_num + 0.5,
        ));
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

    /// Recursively perform an MCTS iteration.
    ///
    /// XXX A non-recursive implementation would probably be faster.
    /// XXX But how to keep &mut pointers to all our parents while
    /// XXX we fiddle with our leaf node?
    pub fn iteration(&mut self, game: &mut Chess, c: f32, rng: &mut SmallRng) -> f32 {
        let delta = match self.state {
            NodeState::LeafNode => game.reward(),
            NodeState::FullyExpanded => {
                // Choose and recurse into child...
                let child = self.best_child(c);
                game.make_move(&child.action.unwrap());
                child.iteration(game, c, rng)
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                if allowed_actions.len() == 0 || game.is_insufficient_material() {
                    self.state = NodeState::LeafNode;
                    return self.iteration(game, c, rng);
                }
                let candidate_actions = self.candidate_actions(allowed_actions);
                if candidate_actions.len() == 0 {
                    self.state = NodeState::FullyExpanded;
                    return self.iteration(game, c, rng);
                }
                let mut child = self.expand(candidate_actions, rng);
                game.make_move(&child.action.unwrap());
                let delta = playout(rng, game).reward();
                child.nn += 1.;
                child.nq += delta;
                delta
            }
        };
        self.nn += 1.;
        self.nq += delta;
        delta
    }
}

impl fmt::Display for TreeNode {
    /// Output a nicely indented tree
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Nested definition for recursive formatting
        fn fmt_subtree(f: &mut fmt::Formatter, node: &TreeNode, indent_level: i32) -> fmt::Result {
            for _ in 0..indent_level {
                try!(f.write_str("    "));
            }
            match node.action {
                Some(a) => try!(writeln!(
                    f,
                    "{}. {} q={} n={} s={}",
                    node.move_num,
                    a,
                    node.total_q(),
                    node.total_n(),
                    node.score()
                )),
                None => try!(writeln!(
                    f,
                    "{}. Root q={} n={} s={}",
                    node.move_num,
                    node.total_q(),
                    node.total_n(),
                    node.score()
                )),
            }
            for child in &node.children {
                try!(fmt_subtree(f, child, indent_level + 1));
            }
            write!(f, "")
        }

        fmt_subtree(f, self, 0)
    }
}

#[derive(Debug, Copy, Clone)]
/// Store and process some simple statistical information about NodeTrees.
pub struct TreeStatistics {
    nodes: i32,
    min_depth: i32,
    max_depth: i32,
}

impl TreeStatistics {
    fn merge(child_stats: &Vec<TreeStatistics>) -> TreeStatistics {
        if child_stats.len() == 0 {
            TreeStatistics {
                nodes: 1,
                min_depth: 0,
                max_depth: 0,
            }
        } else {
            TreeStatistics {
                nodes: child_stats.iter().fold(0, |sum, child| sum + child.nodes),
                min_depth: 1 + child_stats
                    .iter()
                    .fold(i32::MAX, |depth, child| min(depth, child.min_depth)),
                max_depth: 1 + child_stats
                    .iter()
                    .fold(0, |depth, child| max(depth, child.max_depth)),
            }
        }
    }
}
//////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
/// Represents an ensamble of MCTS trees.
///
/// For many applications we need to work with ensambles because we use
/// determinization.
pub struct MCTS {
    iterations_per_ms: f32,
    starting_seed: u8,
}

impl MCTS {
    /// Create a new MCTS solver.
    pub fn new(starting_seed: u8) -> MCTS {
        MCTS {
            iterations_per_ms: STARTING_ITERATIONS_PER_MS,
            starting_seed: starting_seed,
        }
    }

    /// Return basic statistical data about the current MCTS tree.
    ///
    /// XXX Note: The current implementation considers the ensemble
    /// to be a tree layer. In other words tree depth and number of
    /// nodes are all one too large.
    pub fn tree_statistics(&self, roots: &Vec<TreeNode>) -> TreeStatistics {
        let child_stats = roots
            .iter()
            .map(|c| c.tree_statistics())
            .collect::<Vec<_>>();
        TreeStatistics::merge(&child_stats)
    }

    /// Perform n_samples MCTS iterations.
    pub fn search(
        &mut self,
        root: TreeNode,
        game: &Chess,
        ensemble_size: usize,
        n_samples: usize,
        c: f32,
    ) -> TreeNode {
        // Iterate over ensemble and perform MCTS iterations
        let handles: Vec<JoinHandle<TreeNode>> = (0..ensemble_size)
            .map(|thread_num| {
                let thread_game = game.clone();
                let mut thread_root = root.clone();
                let mut rng = seeded_rng(self.starting_seed + thread_num as u8);
                thread::spawn(move || {
                    //Run iterations with playouts for this time slice
                    for _ in 0..n_samples {
                        thread_root.iteration(&mut thread_game.clone(), c, &mut rng);
                    }
                    // println!("root: {}", root);
                    thread_root
                })
            })
            .collect();

        let thread_roots = handles
            .into_iter()
            .map(|th| th.join().expect("panicked joining threads"))
            .collect();

        merge_trees(root, thread_roots)
    }
    pub fn search_time(
        &mut self,
        root: TreeNode,
        game: &Chess,
        ensemble_size: usize,
        time_per_move_ms: f32,
        c: f32,
    ) -> TreeNode {
        let mut samples_total = 0;
        let t0 = Instant::now();

        //TODO MAKE ITERATIONS / SEC saved between runs
        let mut n_samples = (self.iterations_per_ms * time_per_move_ms)
            .max(10.)
            .min(100.) as usize;

        let mut new_root = root;
        while n_samples >= 5 {
            new_root = self.search(new_root, game, ensemble_size, n_samples, c);
            samples_total += n_samples;

            let time_spend = t0.elapsed().as_millis() as f32;
            self.iterations_per_ms = (samples_total as f32) / time_spend;

            let time_left = time_per_move_ms - time_spend;
            n_samples = (self.iterations_per_ms * time_left).max(0.).min(100.) as usize;
        }

        println!("iterations_per_ms: {}", self.iterations_per_ms);

        new_root
    }
}

pub trait Coefficient {
    fn coefficient(&self) -> f32;
}
impl Coefficient for Color {
    fn coefficient(&self) -> f32 {
        match &self {
            Color::Black => -1.,
            Color::White => 1.,
        }
    }
}

#[cfg(test)]
mod tests {
    //     //use std::num::traits::*;
    //     use test::Bencher;
    //
    use mcts::*;
    use shakmaty::fen::Fen;

    #[test]
    fn search_deterministic() {
        fn run_search() -> TreeNode {
            let mut mcts = MCTS::new(1);
            let game = &Chess::default();
            let root = TreeNode::new_root(game, 0.5);
            mcts.search(root, game, 4, 10000, 0.50)
        }
        let a = run_search();
        let b = run_search();
        let c = run_search();
        // println!("{}\n{}", a.tree_statistics(), b.tree_statistics());
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    fn run_search_mate_in_7_deterministic() {
        fn run_search() -> TreeNode {
            let setup: Fen = "rn3rk1/pbppq1pp/1p2pb2/4N2Q/3PN3/3B4/PPP2PPP/R3K2R w KQ - 6 11"
                .parse()
                .unwrap();
            let game: Chess = setup.position().unwrap();
            let mut mcts = MCTS::new(1);
            let root = TreeNode::new_root(&game, 0.5);
            mcts.search(root, &game, 4, 10000, 0.50)
        }
        let a = run_search();
        let b = run_search();
        let c = run_search();
        // println!("{}\n{}", a.tree_statistics(), b.tree_statistics());
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    //
    //
    //     #[bench]
    //     fn bench_expected(b: &mut Bencher) {
    //         let game = MiniGame::new();
    //         b.iter(|| expected_reward(&game, 100))
    //     }
    //
    //     #[bench]
    //     fn bench_search(b: &mut Bencher) {
    //         let game = MiniGame::new();
    //         let mut mcts = MCTS::new(&game, 1);
    //
    //         b.iter(|| mcts.search(10, 1.0))
    //     }
    //
}
