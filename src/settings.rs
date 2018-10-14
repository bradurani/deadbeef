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
    pub starting_iterations_per_ms: f32,
}

impl Settings {
    pub fn game_default() -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: 1000,
            ensemble_size: 4,
            c: 0.25,
            starting_seed: 1,
            starting_iterations_per_ms: 0.5,
        }
    }

    //#[cfg(test)] //need to figure out how to not compile this outside tests
    pub fn test_default() -> Settings {
        Settings::test_default_with_seed(1)
    }

    pub fn test_default_with_seed(seed: u8) -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: 1000000,
            ensemble_size: 1,
            c: 0.25,
            starting_seed: seed,
            starting_iterations_per_ms: 0.5,
        }
    }
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
