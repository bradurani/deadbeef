use search_strategy::*;
use search_threaded::*;
use settings::*;
use show_thinking::*;
use state::*;
use stats::*;
use std::time::Duration;
use tree_node::*;

pub struct SearchTime {
    pub ms: Duration,
}

impl SearchStrategy for SearchTime {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        let mut new_root = state.root;

        for n in 0..100000 {
            if !new_root.is_searchable() || stats.elapsed() >= self.ms {
                break;
            }
            new_root = search_threaded(new_root, stats, settings);
            show_thinking(&new_root, &stats, &settings, n);
        }
        new_root
    }
}
