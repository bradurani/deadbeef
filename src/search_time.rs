use mcts::*;
use search_strategy::*;
use search_threaded_batch::*;
use settings::*;
use state::*;
use stats::*;
use std::time::Duration;

pub struct SearchTime {
    pub ms: Duration,
}

impl SearchStrategy for SearchTime {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        let mut new_root = state.root;

        loop {
            if new_root.has_outcome() {
                break;
            }

            new_root = search_threaded(new_root, &state.position, stats, settings);

            if stats.elapsed() >= self.ms {
                break;
            }
        }
        new_root
    }
}
