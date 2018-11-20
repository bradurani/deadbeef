use mcts::*;
use search_strategy::*;
use search_threaded::*;
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
            if new_root.is_decisive() {
                break;
            }

            new_root = search_threaded(new_root, stats, settings);

            if stats.elapsed() >= self.ms {
                break;
            }
        }
        println!("");
        new_root
    }
}
