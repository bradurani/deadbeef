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
        // don't add total time since we use a separate timer at each
        // stat level
    }

    pub fn add_thread_stats(&mut self, run_stats: &RunStats, thread_count: usize) {
        self.nodes_created += run_stats.nodes_created;
        self.iterations += run_stats.iterations;
        self.playouts += run_stats.playouts;
        self.playout_moves += run_stats.playout_moves;
        self.tree_merges += run_stats.tree_merges;
        self.maxouts += run_stats.maxouts;
        self.samples += run_stats.samples;
        self.sample_batches += run_stats.sample_batches;
        self.playout_time += run_stats.playout_time / thread_count as u64;
        self.tree_merge_time += run_stats.tree_merge_time / thread_count as u64;
        // don't add total time since we use a separate timer at each
        // stat level
    }

    pub fn tree_merge_time_pct(&self) -> f64 {
        self.tree_merge_time as f64 / self.total_time as f64 * 100.
    }

    pub fn playout_time_pct(&self) -> f64 {
        self.playout_time as f64 / self.total_time as f64 * 100.
    }

    pub fn other_time(&self) -> u64 {
        self.total_time - self.tree_merge_time - self.playout_time
    }

    pub fn other_time_pct(&self) -> f64 {
        self.other_time() as f64 / self.total_time as f64 * 100.
    }
}

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n{} moves / {} playouts (avg: {}) with {} maxouts\n\
             {} nodes / {} iterations. {} samples / {} batches\n\
             {} tree_merges\n\
             time in playouts: {} ({:.*}%), tree_merge: {} ({:.*}%), other: {}({:.*}%), total: {}\n",
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
            self.tree_merges.separated_string(),
            self.playout_time.separated_string(),
            1, self.playout_time_pct(),
            self.tree_merge_time.separated_string(),
            1, self.tree_merge_time_pct(),
            self.other_time().separated_string(),
            1, self.other_time_pct(),
            self.total_time
        )
    }
}
