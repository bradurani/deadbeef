use mcts::*;
use separator::Separatable;
use std::cmp::{max, min};
use std::fmt;
use std::i32;

#[derive(Debug, Copy, Clone)]
pub struct TreeStats {
    nodes: i32,
    min_depth: i32,
    max_depth: i32,
    ns: i32,
}

impl TreeStats {
    pub fn tree_stats(root: &TreeNode) -> TreeStats {
        let child_stats = root
            .children
            .iter()
            .map(|c| TreeStats::tree_stats(c))
            .collect::<Vec<_>>();
        TreeStats::merge(&child_stats)
    }

    fn merge(child_stats: &Vec<TreeStats>) -> TreeStats {
        if child_stats.len() == 0 {
            TreeStats {
                nodes: 1,
                min_depth: 0,
                max_depth: 0,
                ns: 0,
            }
        } else {
            TreeStats {
                //TODO very inefficient
                ns: child_stats.iter().fold(0, |sum, child| sum + child.ns),
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

#[derive(Debug, Copy, Clone)]
pub struct RunStats {
    pub nodes_created: u64,
    pub iterations: u64,
    pub playouts: u64,
    pub playout_moves: u64,
    pub maxouts: u64,
    pub samples: u64,
    pub sample_batches: u64,
}

impl RunStats {
    pub fn new() -> RunStats {
        RunStats {
            nodes_created: 0,
            iterations: 0,
            playouts: 0,
            playout_moves: 0,
            maxouts: 0,
            samples: 0,
            sample_batches: 0,
        }
    }

    pub fn add(&mut self, run_stats: &RunStats) {
        self.nodes_created += run_stats.nodes_created;
        self.iterations += run_stats.iterations;
        self.playouts += run_stats.playouts;
        self.playout_moves += run_stats.playout_moves;
        self.maxouts += run_stats.maxouts;
        self.samples += run_stats.samples;
        self.sample_batches += run_stats.sample_batches;
    }
}

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} moves / {} playouts (avg: {}) with {} maxouts. \
             {} nodes / {} iterations. {} samples / {} batches",
            self.playout_moves.separated_string(),
            self.playouts.separated_string(),
            if self.playouts == 0 {
                0
            } else {
                self.playout_moves / self.playouts
            },
            self.maxouts.separated_string(),
            self.nodes_created.separated_string(),
            self.iterations.separated_string(),
            self.samples.separated_string(),
            self.sample_batches.separated_string(),
        )
    }
}
