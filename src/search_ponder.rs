use mcts::*;
use search_strategy::*;
use search_threaded_batch::*;
use settings::*;
use state::*;
use stats::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct SearchPonder {
    pub waiting_for_opponent: Arc<AtomicBool>,
}

impl SearchStrategy for SearchPonder {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        let mut new_root = state.root;
        while self.waiting_for_opponent.load(Ordering::Relaxed) {
            if new_root.has_outcome() {
                break;
            }
            new_root = search_threaded(new_root, &state.position, stats, &settings);
        }
        new_root
    }
}
