use mcts::*;
use std::cmp::{max, min};
use std::i32;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
pub struct TreeStats {
    pub nodes: i32,
    pub min_depth: i32,
    pub max_depth: i32,
    pub ns: i32,
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

#[derive(Debug, Copy, Clone, Default)]
pub struct RunStats {
    pub nodes_created: u64,
    pub iterations: u64,
    pub batches: u64,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub leaf_nodes: u64,
}

impl RunStats {
    pub fn add(&mut self, run_stats: &RunStats) {
        self.nodes_created += run_stats.nodes_created;
        self.iterations += run_stats.iterations;
        self.leaf_nodes += run_stats.leaf_nodes;
    }

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop_timer(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn elapsed(&self) -> Duration {
        self.end_time.unwrap_or(Instant::now()) - self.start_time.unwrap()
    }

    pub fn nodes_per_second(&self) -> u128 {
        (self.nodes_created as u128 * 1000000000) / self.elapsed().as_nanos();
    }
}
