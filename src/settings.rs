use args::*;
use search_strategy::*;

#[derive(Debug, Clone)]
pub struct Settings {
    pub max_threads: u16,
    pub c: f32,
    pub starting_seed: u8,
    pub search_type: SearchType,
    pub batch_size: u32,
    // TODO move out
    pub max_tree_display_depth: u8,
    pub print_tree: bool,
    pub show_thinking: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            max_threads: 12,
            c: 0.5,
            starting_seed: 1,
            search_type: SearchType::Time(1000),
            batch_size: 100,
            max_tree_display_depth: parse_max_tree_display_depth(),
            print_tree: parse_print_tree(),
            show_thinking: false,
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

    pub fn test_lib_default() -> Settings {
        Settings {
            search_type: SearchType::Iterations(10000),
            max_threads: 1,
            show_thinking: false,
            ..Default::default()
        }
    }
}
