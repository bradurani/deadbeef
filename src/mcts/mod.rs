extern crate shakmaty;

use std::fmt;
use std::i32;
use std::f32;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::ops::Not;
use shakmaty::*;

use std::time::{Instant};

use utils::{choose_random};

const MAX_PLAYOUT_MOVES: u32=4000;

/// A `Game` represets a game state.
///
/// It is important that the game behaves fully deterministic,
/// e.g. it has to produce the same game sequences
pub trait Game : Clone {

    /// Return a list with all allowed actions given the current game state.
    fn allowed_actions(&self) -> Vec<Move>;

    /// Change the current game state according to the given action.
    fn make_move(&mut self, action: &Move);

    /// Reward for the player when reaching the current game state.
    fn reward(&self) -> f32;

    /// Derterminize the game
    fn set_rng_seed(&mut self, seed: u32);
}

/// Perform a random playout.
///
/// Start with an initial game state and perform random actions from
/// until a game-state is reached that does not have any `allowed_actions`.
pub fn playout(initial: &Chess) -> Chess {

    let mut game = initial.clone();

    let mut potential_moves = game.allowed_actions();

    let mut num_moves = 0;
    while potential_moves.len() > 0 {
        num_moves += 1;
        if num_moves >= MAX_PLAYOUT_MOVES {
            eprintln!("REACHED MAX PLAYOUT LENGTH");
            break;
        }

        let action = choose_random(&potential_moves).clone();
        game.make_move(&action);
        potential_moves = game.allowed_actions();
    }
    game
}

// /// Calculate the expected reward based on random playouts.
// pub fn expected_reward<G: Chess, A: GameAction>(game: &G, n_samples: usize) -> f32 {
//     let mut score_sum: f32 = 0.0;
//
//     for _ in 0..n_samples {
//         score_sum += playout(game).reward();
//     }
//     (score_sum as f32) / (n_samples as f32)
// }


//////////////////////////////////////////////////////////////////////////

#[derive(Debug,Copy,Clone)]
enum NodeState {
    LeafNode, FullyExpanded, Expandable
}

#[derive(Debug)]
pub struct TreeNode {
    action: Option<Move>,                  // how did we get here
    children: Vec<TreeNode>,         // next steps we investigated
    state: NodeState,                   // is this a leaf node? fully expanded?
    turn: Color,                          //which player made this move
    move_num: f32,
    n: f32, q: f32                      // statistics for this game state
}

impl TreeNode{

    /// Create and initialize a new TreeNode
    ///
    /// Initialize q and n t to be zero; childeren list to
    /// be empty and set the node state to Expandable.
    pub fn new(action: Option<Move>, turn: Color, move_num: f32) -> TreeNode {
        TreeNode{
            action: action,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: turn,
            move_num: move_num,
            n: 0., q: 0. }
    }

    pub fn starting() -> TreeNode{
        TreeNode{
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: Color::Black, // So we switch to White for move 1
            move_num: 0.5, //So we increment to 1 for move 1
            n: 0., q:0. }
    }

    /// Gather some statistics about this subtree
    pub fn tree_statistics(&self) -> TreeStatistics {
        let child_stats = self.children.iter()
            .map(|c| c.tree_statistics())
            .collect::<Vec<_>>();
        TreeStatistics::merge(child_stats)
    }

    /*
    /// XXX
    pub fn merge_nodes(nodes: Vec<TreeNode<A>>, depth: usize) -> TreeNode<A> {
    }
    */

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, c: f32, child_turn: &Color) -> &mut TreeNode {
        let mut best_value :f32 = f32::NEG_INFINITY;
        let mut best_child :Option<&mut TreeNode> = None;

        for child in &mut self.children {
            let color_coefficient = color_coefficient(&child_turn);
            let value = (color_coefficient * child.q) /
                child.n + c*(2.*self.n.ln()/child.n).sqrt();
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
    ///
    /// XXX Use HashSet? Use iterators? XXX
    pub fn expand(&mut self, game: &Chess) -> Option<&mut TreeNode> {

        // What are our options given the current game state?
        let allowed_actions = game.allowed_actions();
        if allowed_actions.len() == 0 {
            self.state = NodeState::LeafNode;
            return None;
        }

        // Get a list with all the actions we tried alreday
        let mut child_actions : Vec<Move> = Vec::new();
        for child in &self.children {
            child_actions.push(child.action.expect("Child node without action"));
        }

        // Find untried actions
        let mut candidate_actions = Vec::new();
        for action in &allowed_actions {
            if !child_actions.contains(action) {
                candidate_actions.push(action);
            }
        }

        if candidate_actions.len() == 1 {
            self.state = NodeState::FullyExpanded;
        }

        // Select random actions
        let action = *choose_random(&candidate_actions).clone();

        self.children.push(TreeNode::new(Some(action), self.turn.not(), self.move_num + 0.5));
        self.children.last_mut()
    }

    /// Recursively perform an MCTS iteration.
    ///
    /// XXX A non-recursive implementation would probably be faster.
    /// XXX But how to keep &mut pointers to all our parents while
    /// XXX we fiddle with our leaf node?
    pub fn iteration(&mut self, game: &mut Chess, c: f32) -> f32 {
        let delta = match self.state {
            NodeState::LeafNode => {
                game.reward()
            },
            NodeState::FullyExpanded => {
                // Choose and recurse into child...
                let child_turn = &self.turn.not();
                let child = self.best_child(c, child_turn);
                game.make_move(&child.action.unwrap());
                child.iteration(game, c)
            },
            NodeState::Expandable => {
                let child = self.expand(game);
                match child {
                    Some(child) => {           // We expanded our current node...
                        game.make_move(&child.action.unwrap());
                        let delta = playout(game).reward();
                        child.n += 1.;
                        child.q += delta;
                        delta
                    },
                    None => game.reward()      // Could not expand, current node is a leaf node!
                }
            }
        };
        self.n += 1.;
        self.q += delta;
        delta
    }
}


impl fmt::Display for TreeNode {

    /// Output a nicely indented tree
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Nested definition for recursive formatting
        fn fmt_subtree(f: &mut fmt::Formatter, node: &TreeNode, indent_level :i32) -> fmt::Result {
            for _ in 0..indent_level {
                try!(f.write_str("    "));
            }
            let score = node.q / node.n;
            match node.action {
                Some(a)  => try!(writeln!(f, "{}. {} q={} n={} s={}",
                                          node.move_num, a, node.q, node.n, score)),
                None     => try!(writeln!(f, "{}. Root q={} n={} s={}",
                                          node.move_num, node.q, node.n, score))
            }
            for child in &node.children {
                try!(fmt_subtree(f, child, indent_level+1));
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
    fn merge(child_stats: Vec<TreeStatistics>) -> TreeStatistics {
        if child_stats.len() == 0 {
            TreeStatistics {
                nodes: 1,
                min_depth: 0,
                max_depth: 0,
            }
        } else {
            TreeStatistics {
                nodes: child_stats.iter()
                    .fold(0, |sum, child| sum + child.nodes),
                    min_depth: 1 + child_stats.iter()
                        .fold(i32::MAX, |depth, child| min(depth, child.min_depth)),
                        max_depth: 1 + child_stats.iter()
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
    roots: Vec<TreeNode>,
    games: Vec<Chess>,
    iterations_per_ms: f32,
}

impl MCTS {

    /// Create a new MCTS solver.
    pub fn new(game: &Chess, move_num: f32, ensamble_size: usize) -> MCTS {
        let mut roots = Vec::new();
        let mut games = Vec::new();
        for i in 0..ensamble_size {
            let mut game = game.clone();
            let turn = game.turn();
            game.set_rng_seed(i as u32);
            games.push(game);
            roots.push(TreeNode::new(None, turn.not(), move_num));
        }
        MCTS {
            roots: roots,
            games: games,
            iterations_per_ms: 0.1
        }
    }

    /// Return basic statistical data about the current MCTS tree.
    ///
    /// XXX Note: The current implementation considers the ensemble
    /// to be a tree layer. In other words tree depth and number of
    /// nodes are all one too large.
    pub fn tree_statistics(&self) -> TreeStatistics {
        let child_stats = self.roots.iter()
            .map(|c| c.tree_statistics())
            .collect::<Vec<_>>();
        TreeStatistics::merge(child_stats)
    }
    /// Set a new game state for this solver.
    pub fn advance_game(&mut self, game: &Chess, move_num: f32) {
        let ensamble_size = self.games.len();

        let mut roots = Vec::new();
        let mut games = Vec::new();
        for i in 0..ensamble_size {
            let mut game = game.clone();
            let turn = game.turn();
            game.set_rng_seed(i as u32);
            games.push(game);
            roots.push(TreeNode::new(None, turn,  move_num)); //very inefficient. Scraps previous work
        }
        self.games = games;
        self.roots = roots;
    }

    /// Perform n_samples MCTS iterations.
    pub fn search(&mut self, n_samples: usize, c: f32) {
        let ensamble_size = self.games.len();

        // Iterate over ensamble and perform MCTS iterations
        for e in 0..ensamble_size {
            // println!("ensemble: {}", e);
            let game = &self.games[e];
            let root = &mut self.roots[e];

            // Perform MCTS iterations
            for _ in 0..n_samples {
                let mut this_game = game.clone();
                root.iteration(&mut this_game, c);
            }
            // println!("root: {}", root);
        }
    }

    /// Perform MCTS iterations for the given time budget (in s).
    pub fn search_time(&mut self, time_per_move_ms: f32, c: f32) {
        let mut samples_total = 0;
        let t0 = Instant::now();

        let mut n_samples = (self.iterations_per_ms*time_per_move_ms).
            max(10.).
            min(100.) as usize;

        while n_samples >= 5 {
            self.search(n_samples, c);
            samples_total += n_samples;

            let time_spend = t0.elapsed().as_millis() as f32;
            self.iterations_per_ms = (samples_total as f32) / time_spend;

            let time_left = time_per_move_ms - time_spend;
            n_samples = (self.iterations_per_ms*time_left).max(0.).min(100.) as usize;
        }
        println!("iterations_per_ms: {}", self.iterations_per_ms);
    }

    /// Return the best action found so far by averaging over the ensamble.
    pub fn best_action(&self) -> Option<Move> {
        let ensamble_size = self.games.len();

        // Merge ensamble results
        let mut n_values = HashMap::<Move, f32>::new();
        let mut q_values = HashMap::<Move, f32>::new();

        for e in 0..ensamble_size {
            let root = &self.roots[e];

            for child in &root.children {
                let action = child.action.unwrap();

                let n = n_values.entry(action).or_insert(0.);
                let q = q_values.entry(action).or_insert(0.);

                *n += child.n;
                *q += child.q;
            }
        }

        let color = self.games.first().unwrap().turn();
        let color_coefficient = color_coefficient(&color);

        // Find best action
        let mut best_action: Option<Move> = None;
        let mut best_value: f32 = f32::NEG_INFINITY;
        for (action, n) in &n_values {
            let q = q_values.get(action).unwrap();
            let value = (color_coefficient * q) / n;
            if value > best_value {
                best_action = Some(*action);
                best_value = value;
            }
        }
        println!("Best value for {:?}: {}", color, color_coefficient * best_value);

        best_action
    }
}

fn color_coefficient(color: &Color) -> f32 {
    match color {
        Color::Black => -1.,
        Color::White => 1.
    }
}


impl fmt::Display for MCTS {

    /// Output a nicely indented tree
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Ensable of {} trees:", self.roots.len()));
        //for root in &self.roots {
        //    try!(root.fmt(f));
        //}
        write!(f, "")
    }
}


/////////////////////////////////////////////////////////////////////////////
// Unittests

// #[cfg(test)]
// mod tests {
//     //use std::num::traits::*;
//     use test::Bencher;
//
//     use mcts::*;
//     use minigame::MiniGame;
//
//     #<{(|
//     // Are the given
//     fn allmost_equal<T: Float>(a: T, b: T) -> bool {
//         let rtol = 1e-6;
//         // Shortcut for inf and neg_inf
//         if (a == b) { return true };
//         let a_abs = a.abs();
//         let b_abd = b.abs();
//         let diff = (a-b).abs();
//         diff <= tol * a_abs.max(b_abs)
//     }
//     |)}>#
//
//     #[test]
//     fn test_playout() {
//         let game = MiniGame::new();
//         let game = playout(&game);
//         println!("Final: {:?}", game);
//     }
//
//     #[test]
//     fn test_expand() {
//         let game = MiniGame::new();
//         let mut node = TreeNode::new(None);
//
//         node.expand(&game);
//         node.expand(&game);
//         {
//             let v = node.expand(&game).unwrap();
//             v.expand(&game);
//         }
//
//         println!("After some expands:\n{}", node);
//     }
//
//     #[test]
//     fn test_tree_statistics() {
//         let game = MiniGame::new();
//         let mut mcts = MCTS::new(&game, 2);
//
//         mcts.search(50, 1.);
//
//         let stats = mcts.tree_statistics();
//
//         println!("{:?}", stats);
//     }
//
//     #<{(|
//     #[test]
//     fn test_mcts() {
//         let game = MiniGame::new();
//         let mut mcts = MCTS::new(&game, 1);
//         //println!("MCTS on new game: {:?}", mcts);
//         for i in 0..5 {
//             mcts.root.iteration(&mut game.clone(), 1.0);
//             println!("After {} iteration(s):\n{}", i, mcts);
//         }
//     }|)}>#
//
//     #[test]
//     fn test_search() {
//         let game = MiniGame::new();
//         let mut mcts = MCTS::new(&game, 2);
//
//         mcts.search(50, 1.);
//
//         println!("Search result: {:?}", mcts.best_action());
//     }
//
//     #[test]
//     fn test_search_time() {
//         let game = MiniGame::new();
//         let mut mcts = MCTS::new(&game, 2);
//
//         // Search for ~0.5 seconds
//         let budget_seconds = 0.5;
//
//         let t0 = time::now();
//         mcts.search_time(budget_seconds, 1.);
//
//         let time_spent = (time::now() - t0).num_milliseconds();
//
//         println!("Time spent in search_time: {}", time_spent);
//
//         // Check we really spent ~500 ms searching...
//         assert!(time_spent > 200);
//         assert!(time_spent < 700);
//     }
//
//     #[bench]
//     fn bench_playout(b: &mut Bencher) {
//         let game = MiniGame::new();
//         b.iter(|| playout(&game))
//     }
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
// }
