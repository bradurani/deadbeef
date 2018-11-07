use mcts::*;
use std::cmp::{max, min};
use std::i32;

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
    pub playouts: u64,
    pub playout_moves: u64,
    pub tree_merges: u64,
    pub maxouts: u64,
    pub samples: u64,
    pub sample_batches: u64,
    pub playout_time: u64,
    pub tree_merge_time: u64,
    pub total_time: u64,
    pub leaf_nodes: u64,
}

impl RunStats {
    pub fn add(&mut self, run_stats: &RunStats) {
        self.nodes_created += run_stats.nodes_created;
        self.iterations += run_stats.iterations;
        self.playouts += run_stats.playouts;
        self.playout_moves += run_stats.playout_moves;
        self.tree_merges += run_stats.tree_merges;
        self.maxouts += run_stats.maxouts;
        self.samples += run_stats.samples;
        self.sample_batches += run_stats.sample_batches;
        self.playout_time += run_stats.playout_time;
        self.tree_merge_time += run_stats.tree_merge_time;
        self.leaf_nodes += run_stats.leaf_nodes;
        // don't add total time since we use a separate timer at each
        // stat level
    }

    pub fn add_thread_stats(&mut self, run_stats: &RunStats, _thread_count: u16) {
        self.nodes_created += run_stats.nodes_created;
        self.iterations += run_stats.iterations;
        self.playouts += run_stats.playouts;
        self.playout_moves += run_stats.playout_moves;
        self.tree_merges += run_stats.tree_merges;
        self.maxouts += run_stats.maxouts;
        self.samples += run_stats.samples;
        self.sample_batches += run_stats.sample_batches;
        self.playout_time += run_stats.playout_time; // thread_count as u64;
        self.tree_merge_time += run_stats.tree_merge_time; // thread_count as u64;
        self.leaf_nodes += run_stats.leaf_nodes;
        // don't add total time since we use a separate timer at each
        // stat level
    }

    pub fn tree_merge_time_pct(&self) -> f64 {
        if self.total_time == 0 {
            0.
        } else {
            self.tree_merge_time as f64 / self.total_time as f64 * 100.
        }
    }

    pub fn playout_time_pct(&self) -> f64 {
        if self.total_time == 0 {
            0.
        } else {
            self.playout_time as f64 / self.total_time as f64 * 100.
        }
    }

    pub fn other_time(&self) -> u64 {
        (self.total_time as i64 - self.tree_merge_time as i64 - self.playout_time as i64) as u64
    }

    pub fn other_time_pct(&self) -> f64 {
        if self.total_time == 0 {
            0.
        } else {
            self.other_time() as f64 / self.total_time as f64 * 100.
        }
    }
}
