use mcts::*;
use search_iterations::*;
use search_ponder::*;
use search_time::*;
use settings::*;
use state::*;
use stats::*;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum SearchType {
    Iterations(u32),
    Time(u32),
    Ponder(Arc<AtomicBool>),
}

pub trait SearchStrategy {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode;
}

pub fn search_with_strategy(state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
    match settings.search_type.clone() {
        SearchType::Iterations(n_iterations) => {
            let strategy = SearchIterations { n_iterations };
            strategy.search(state, stats, &settings)
        }
        SearchType::Time(ms) => {
            let strategy = SearchTime { ms: ms };
            strategy.search(state, stats, &settings)
        }
        SearchType::Ponder(atomic_bool) => {
            let strategy = SearchPonder {
                waiting_for_opponent: atomic_bool,
            };
            strategy.search(state, stats, &settings)
        }
    }
}
