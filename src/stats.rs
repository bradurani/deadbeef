use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct RunStats {
    pub nodes_created: u64,
    pub iterations: u64,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub leaf_nodes: u64,
    pub evals: u64,
    pub playout_leaves: u64,
    pub mcts_depth: usize,
    pub mcts_max_depth: usize,
    pub playout_max_depth: usize,
    pub q_max_depth: usize,
}

impl RunStats {
    pub fn add(&mut self, run_stats: &RunStats) {
        self.nodes_created += run_stats.nodes_created;
        self.iterations += run_stats.iterations;
        self.leaf_nodes += run_stats.leaf_nodes;
        self.evals += run_stats.evals;
        self.playout_leaves += run_stats.playout_leaves;
        self.mcts_max_depth = self.mcts_max_depth.max(run_stats.mcts_max_depth);
        self.playout_max_depth = self.playout_max_depth.max(run_stats.playout_max_depth);
        self.q_max_depth = self.q_max_depth.max(run_stats.q_max_depth);
    }

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop_timer(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn elapsed(&self) -> Duration {
        self.end_time.unwrap_or(Instant::now()) - self.start_time.unwrap()
    }

    pub fn evals_per_second(&self) -> u64 {
        ((self.evals as u128 * 1000000000) / self.elapsed().as_nanos()) as u64
    }

    pub fn q_percent(&self) -> u64 {
        self.evals / self.playout_leaves * 100
    }

    pub fn increase_mcts_depth(&mut self) {
        self.mcts_depth += 1;
        self.mcts_max_depth = self.mcts_max_depth.max(self.mcts_depth);
    }

    pub fn decrease_mcts_depth(&mut self) {
        self.mcts_depth -= 1;
    }

    pub fn record_playout_depth(&mut self, depth: usize) {
        self.playout_max_depth = self.playout_max_depth.max(depth);
    }

    pub fn record_q_depth(&mut self, depth: usize) {
        self.q_max_depth = self.q_max_depth.max(depth);
    }

    pub fn max_depth(&self) -> usize {
        self.mcts_max_depth + self.playout_max_depth + self.q_max_depth
    }
}
