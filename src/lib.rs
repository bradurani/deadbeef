#![feature(duration_as_u128)]
// #![feature(tool_attributes)] // for rustfmt directives

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate log;
extern crate core;
extern crate env_logger;
extern crate pad;
extern crate rand;
extern crate separator;
extern crate shakmaty;
extern crate twox_hash;
pub mod args;
pub mod display;
pub mod emojify;
pub mod engine;
pub mod eval;
pub mod game;
pub mod logger;
pub mod mcts;
pub mod pgn;
pub mod play;
pub mod playout;
pub mod repetition_detector;
pub mod search_iterations;
pub mod search_ponder;
pub mod search_strategy;
pub mod search_threaded_batch;
pub mod search_time;
pub mod settings;
pub mod setup;
pub mod show_thinking;
pub mod state;
pub mod stats;
pub mod time_remaining;
pub mod uct;
pub mod utils;
pub mod xboard;
