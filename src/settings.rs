use shakmaty::Chess;
use std::isize;

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
    pub search_type: SearchType,
    pub max_batch_size: usize,
    pub min_batch_size: usize,
}

impl Settings {
    pub fn game_default() -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: 500.0,
            n_samples: -1,
            ensemble_size: 8,
            c: 0.25,
            starting_seed: 1,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Time,
            max_batch_size: 200,
            min_batch_size: 4,
        }
    }

    //#[cfg(test)] //need to figure out how to not compile this outside tests
    pub fn test_default() -> Settings {
        Settings::test_default_with_seed(1)
    }

    pub fn test_mate_default() -> Settings {
        Settings::test_mate_default_with_seed(1)
    }

    pub fn test_default_with_seed(seed: u8) -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: 50,
            ensemble_size: 8,
            c: 0.25,
            starting_seed: seed,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Steps,
            max_batch_size: 100,
            min_batch_size: 4,
        }
    }

    pub fn test_mate_default_with_seed(seed: u8) -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: isize::MAX,
            ensemble_size: 8,
            c: 1.,
            starting_seed: seed,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Mate,
            max_batch_size: 100,
            min_batch_size: 4,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SearchType {
    Steps,
    Time,
    Mate,
}
