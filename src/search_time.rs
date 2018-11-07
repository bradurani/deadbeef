use mcts::*;
use search_strategy::*;
use search_threaded_batch::*;
use settings::*;
use state::*;
use stats::*;
use std::time::Instant;

pub struct SearchTime {
    pub ms: u32,
}

impl SearchStrategy for SearchTime {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        let move_start_time = Instant::now();

        let mut new_root = state.root;

        loop {
            if new_root.has_outcome() {
                break;
            }

            let batch_stats: RunStats = Default::default();
            new_root = search_threaded(new_root, &state.position, stats, settings);

            let move_time_spent = move_start_time.elapsed().as_millis() as u32;

            let time_left = self.ms - move_time_spent;
            if time_left < 50 {
                break;
            }

            stats.add(&batch_stats);
        }

        new_root
    }
}
