use shakmaty::Chess;

#[derive(Debug, Clone)]
pub struct Settings {
    pub starting_position: Chess,
    pub starting_move_num: f32,
    pub ensemble_size: usize,
    pub time_per_move_ms: f32,
    pub c: f32,
    pub starting_seed: u8,
    pub n_samples: isize,
}

impl Settings {
    pub fn use_steps(&self) -> bool {
        if self.time_per_move_ms == -1. {
            assert!(self.n_samples > 0);
            return true;
        } else {
            assert!(self.n_samples == -1);
            assert!(self.time_per_move_ms > 0.);
            return false;
        }
    }
}
