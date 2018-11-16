use args::*;
use search_strategy::*;
use std::time::*;

#[derive(Debug, Clone)]
pub struct Settings {
    pub max_threads: u16,
    pub c: f32,
    pub starting_seed: u8,
    pub search_type: SearchType,
    pub batch_size: u32,
    // TODO move out
    pub max_tree_display_depth: Option<u8>,
    pub max_tree_display_length: Option<u8>,
    pub print_tree: bool,
    pub log_level: String,
    pub show_thinking: bool,
    pub show_thinking_freq: u64,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            max_threads: 4,
            c: 10.,
            starting_seed: 2,
            search_type: SearchType::Time(Duration::from_millis(5500)),
            batch_size: 100,
            max_tree_display_depth: parse_max_tree_display_depth(),
            max_tree_display_length: parse_max_tree_display_length(),
            print_tree: parse_print_tree(),
            log_level: parse_log_level(),
            show_thinking: true,
            show_thinking_freq: 150,
        }
    }
}

impl Settings {
    //#[cfg(test)] //need to figure out how to not compile this outside tests
    pub fn test_default() -> Settings {
        Settings {
            search_type: SearchType::Iterations(1000000),
            ..Default::default()
        }
    }

    pub fn test_outcome_default() -> Settings {
        Settings {
            search_type: SearchType::Iterations(5000000),
            ..Default::default()
        }
    }

    pub fn test_iteration_default() -> Settings {
        Settings {
            search_type: SearchType::Iterations(1000000),
            max_threads: 0,
            show_thinking: false,
            ..Default::default()
        }
    }

    pub fn test_lib_default() -> Settings {
        Settings {
            search_type: SearchType::Iterations(10000),
            max_threads: 1,
            show_thinking: false,
            ..Default::default()
        }
    }
}
