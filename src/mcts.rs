extern crate shakmaty;

use shakmaty::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::f32;
use std::fmt;
use std::i32;
use std::thread;
use std::thread::JoinHandle;
use std::ops::Not;

use std::time::Instant;

use utils::choose_random;

const MAX_PLAYOUT_MOVES: u32 = 4000;
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
    while potential_moves.len() > 0 && !game.is_insufficient_material() {
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
    pub n: f32,
    pub q: f32,                  // statistics for this game state
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

    fn set_rng_seed(&mut self, _seed: u32) {}
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
            n: 0.,
            q: 0.,
        }
    }

    pub fn new_root(game: &Chess, move_num: f32) -> TreeNode {
        TreeNode {
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: game.turn(),  // So we switch to White for move 1
            move_num: move_num, //So we increment to 1 for move 1
            n: 0.,
            q: 0.,
        }
    }

    pub fn starting() -> TreeNode {
        TreeNode {
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 0.5,
            n: 0.,
            q: 0.,
        }
    }

    pub fn score(&self) -> f32 {
        self.q as f32 / self.n as f32
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

    /*
    /// XXX
    pub fn merge_nodes(nodes: Vec<TreeNode<A>>, depth: usize) -> TreeNode<A> {
    }
    */

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, c: f32) -> &mut TreeNode {
        let mut best_value: f32 = f32::NEG_INFINITY;
        let mut best_child: Option<&mut TreeNode> = None;

        for child in &mut self.children {
            let value =
                (self.turn.coefficient() * child.q) / child.n + c * (2. * self.n.ln() / child.n).sqrt();
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
    pub fn expand(&mut self, candidate_actions: Vec<Move>) -> &mut TreeNode {
        // println!("Candidate Action: {:?}", &candidate_actions);

        let action = *choose_random(&candidate_actions);

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
    pub fn iteration(&mut self, game: &mut Chess, c: f32) -> f32 {
        let delta = match self.state {
            NodeState::LeafNode => game.reward(),
            NodeState::FullyExpanded => {
                // Choose and recurse into child...
                let child = self.best_child(c);
                game.make_move(&child.action.unwrap());
                child.iteration(game, c)
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                if allowed_actions.len() == 0 || game.is_insufficient_material() {
                    self.state = NodeState::LeafNode;
                    return self.iteration(game, c);
                }
                let candidate_actions = self.candidate_actions(allowed_actions);
                if candidate_actions.len() == 0 {
                    self.state = NodeState::FullyExpanded;
                    return self.iteration(game, c);
                }
                let mut child = self.expand(candidate_actions);
                game.make_move(&child.action.unwrap());
                let delta = playout(game).reward();
                child.n += 1.;
                child.q += delta;
                delta
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
        fn fmt_subtree(f: &mut fmt::Formatter, node: &TreeNode, indent_level: i32) -> fmt::Result {
            for _ in 0..indent_level {
                try!(f.write_str("    "));
            }
            match node.action {
                Some(a) => try!(writeln!(
                        f,
                        "{}. {} q={} n={} s={}",
                        node.move_num, a, node.q, node.n, node.score()
                )),
                None => try!(writeln!(
                        f,
                        "{}. Root q={} n={} s={}",
                        node.move_num, node.q, node.n, node.score()
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
}

impl MCTS {
    /// Create a new MCTS solver.
    pub fn new() -> MCTS {
        MCTS {
            iterations_per_ms: STARTING_ITERATIONS_PER_MS,
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
        root: &TreeNode,
        game: &Chess,
        ensemble_size: usize,
        n_samples: usize,
        c: f32,
        ) -> Vec<TreeNode> {
        // Iterate over ensemble and perform MCTS iterations
        let handles: Vec<JoinHandle<TreeNode>> = (0..ensemble_size)
            .map(|_e| {
                let thread_game = game.clone();
                let mut thread_root = root.clone();
                thread::spawn(move || {
                    // Perform MCTS iterations
                    for _ in 0..n_samples {
                        thread_root.iteration(&mut thread_game.clone(), c);
                    }
                    // println!("root: {}", root);
                    thread_root
                })
            })
        .collect();

        handles.into_iter().map(|th| th.join().unwrap()).collect()
    }

    /// Perform MCTS iterations for the given time budget (in s).
    pub fn search_time(
        &mut self,
        root: TreeNode,
        game: &Chess,
        ensemble_size: usize,
        time_per_move_ms: f32,
        c: f32,
        ) -> Vec<TreeNode> {
        let mut samples_total = 0;
        let t0 = Instant::now();

        //TODO MAKE ITERATIONS / SEC saved between runs
        let mut n_samples = (self.iterations_per_ms * time_per_move_ms)
            .max(10.)
            .min(100.) as usize;

        let mut roots = Vec::new();
        while n_samples >= 5 {
            let thread_roots = self.search(&root, game, ensemble_size, n_samples, c);
            roots.push(thread_roots);
            samples_total += n_samples;

            let time_spend = t0.elapsed().as_millis() as f32;
            self.iterations_per_ms = (samples_total as f32) / time_spend;

            let time_left = time_per_move_ms - time_spend;
            n_samples = (self.iterations_per_ms * time_left).max(0.).min(100.) as usize;
        }
        println!("iterations_per_ms: {}", self.iterations_per_ms);

        roots.into_iter().flat_map(|r| r.into_iter()).collect()
    }

    /// Return the best action found so far by averaging over the ensamble.
    pub fn best_children(&self, roots: Vec<TreeNode>) -> Option<Vec<TreeNode>> {
        let color = roots.first().unwrap().turn;

        let combined_children: Vec<TreeNode> = roots.into_iter().
            flat_map(|r| r.children).collect();

        let mut action_map: HashMap<Move, Vec<TreeNode>> = HashMap::new();

        for r in combined_children {
            let action_nodes = action_map.entry(r.action.unwrap()).or_insert(vec![]);
            action_nodes.push(r);
        }

        let summed_actions: Vec<(&Move, f32)> = action_map
            .iter()
            .map(|(action, nodes)| {
                let score_sum = sum_node_list(nodes.clone(), color.coefficient());
                (action, score_sum)
            }).collect();

        summed_actions.into_iter().max_by(|n1, n2| n1.1.partial_cmp(&n2.1).unwrap())
            .map(|(action, _score)| action_map[action].to_vec())
    }
}

fn sum_node_list(nodes: Vec<TreeNode>, color_coefficient: f32) -> f32{
    nodes.iter().fold(0., |sum, node|{
        println!("sum: {}, color_coefficient: {}, score: {}",
                 sum, color_coefficient, node.score());
        sum + (color_coefficient * node.score())
    })
}

pub trait Coefficient { fn coefficient(&self) -> f32; }
impl Coefficient for Color {
    fn coefficient(&self) -> f32 {
        match &self {
            Color::Black => -1.,
            Color::White => 1.,
        }
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
