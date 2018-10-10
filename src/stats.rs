use mcts::*;
use std::cmp::{max, min};
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
