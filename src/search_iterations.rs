use mcts::*;
use search_strategy::*;
use search_threaded_batch::*;
use settings::*;
use show_thinking::*;
use state::*;
use stats::*;
use std::io;
use std::io::prelude::*;

pub struct SearchIterations {
    pub n_iterations: u32,
}

impl SearchStrategy for SearchIterations {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        debug_assert!(self.n_iterations > settings.batch_size);
        let ply_num = state.ply_num();

        let batches = self.n_iterations / settings.batch_size;

        let mut new_root = state.root;

        stats.start_timer();
        for _i in 0..batches {
            if new_root.has_outcome() {
                return new_root;
            }

            eprint!(".");
            io::stderr().flush().expect("Could not flush stderr");
            new_root = search_threaded(new_root, &state.position, stats, settings);

            stats.batches += 1;
            show_thinking(ply_num, new_root.score(), &stats, &settings);
        }
        stats.stop_timer();
        new_root
    }
}
