use search_strategy::*;
use search_threaded::*;
use settings::*;
use state::*;
use stats::*;
use tree_node::*;

pub struct SearchIterations {
    pub n_iterations: u32,
}

impl SearchStrategy for SearchIterations {
    fn search(&self, state: State, stats: &mut RunStats, settings: &Settings) -> TreeNode {
        let mut new_root = state.root;

        for _i in 0..self.n_iterations {
            if new_root.is_decisive() {
                return new_root;
            }

            new_root = search_threaded(new_root, stats, settings);
        }
        println!("");
        new_root
    }
}
