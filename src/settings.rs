use args::*;
use shakmaty::Chess;
use std::isize;

#[derive(Debug, Clone)]
pub struct Settings {
    pub starting_position: Chess,
    pub starting_move_num: f32,
    pub max_threads: usize,
    pub time_per_move_ms: f32,
    pub c: f32,
    pub starting_seed: u8,
    pub n_samples: isize,
    pub starting_iterations_per_ms: f32,
    pub search_type: SearchType,
    pub max_batch_size: usize,
    pub min_batch_size: usize,
    pub max_tree_display_depth: u8,
    pub print_tree: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            // TODO remove
            starting_position: parse_starting_position(),
            // TODO remove
            starting_move_num: 1.0,
            time_per_move_ms: 5000.0,
            n_samples: -1,
            max_threads: 12,
            c: 0.5,
            starting_seed: 1,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Time,
            max_batch_size: 50,
            min_batch_size: 4,
            max_tree_display_depth: parse_max_tree_display_depth(),
            print_tree: parse_print_tree(),
        }
    }
}

impl Settings {
    //#[cfg(test)] //need to figure out how to not compile this outside tests
    pub fn test_default() -> Settings {
        Settings::test_default_with_seed(1)
    }

    pub fn test_mate_default() -> Settings {
        Settings::test_mate_default_with_seed(1)
    }

    pub fn lib_test_default() -> Settings {
        Settings::test_default_with_seed(1)
    }

    pub fn lib_test_default_with_seed(seed: u8) -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: 100,
            max_threads: 12,
            c: 0.5,
            starting_seed: seed,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Steps,
            max_batch_size: 50,
            min_batch_size: 4,
            max_tree_display_depth: parse_max_tree_display_depth(),
            print_tree: parse_print_tree(),
        }
    }

    pub fn test_default_with_seed(seed: u8) -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: 400000,
            max_threads: 12,
            c: 0.2,
            starting_seed: seed,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Steps,
            max_batch_size: 100,
            min_batch_size: 4,
            max_tree_display_depth: parse_max_tree_display_depth(),
            print_tree: parse_print_tree(),
        }
    }

    pub fn test_mate_default_with_seed(seed: u8) -> Settings {
        Settings {
            starting_position: Chess::default(),
            starting_move_num: 1.0,
            time_per_move_ms: -1.0,
            n_samples: 200000,
            max_threads: 12,
            c: 10.,
            starting_seed: seed,
            starting_iterations_per_ms: 0.5,
            search_type: SearchType::Mate,
            max_batch_size: 100,
            min_batch_size: 4,
            max_tree_display_depth: parse_max_tree_display_depth(),
            print_tree: parse_print_tree(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SearchType {
    Steps,
    Time,
    Mate,
}
