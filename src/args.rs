use std::env;

pub fn parse_max_tree_display_depth() -> u8 {
    env::var("MAX_TREE_DISPLAY_DEPTH")
        .unwrap_or("3".to_string())
        .parse::<u8>()
        .unwrap()
}

pub fn parse_print_tree() -> bool {
    env::var("PRINT_TREE")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .unwrap()
}
