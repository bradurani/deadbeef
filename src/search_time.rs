use mcts::*;
use search_strategy::*;
use search_threaded_batch::*;
use settings::*;
use show_thinking::*;
use state::*;
use stats::*;
use std::io;
use std::io::prelude::*;
use std::time::Duration;

pub struct SearchTime {
    pub ms: Duration,
}

impl SearchStrategy for SearchTime {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        let ply_num = state.ply_num();
        let mut new_root = state.root;

        stats.start_timer();
        loop {
            if new_root.has_outcome() {
                break;
            }

            new_root = search_threaded(new_root, &state.position, stats, settings);
            eprint!(".");
            io::stderr().flush().expect("Could not flush stderr");

            stats.batches += 1;

            show_thinking(ply_num, new_root.score(), &stats, &settings);

            if stats.elapsed() >= self.ms {
                break;
            }
        }
        stats.stop_timer();

        new_root
    }
}
