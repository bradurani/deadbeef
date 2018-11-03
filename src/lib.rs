#![feature(duration_as_u128)]
// #![feature(tool_attributes)] // for rustfmt directives

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate core;
extern crate pad;
extern crate rand;
extern crate separator;
extern crate shakmaty;
extern crate twox_hash;
pub mod args;
pub mod display;
pub mod eval;
pub mod game;
pub mod input;
pub mod mcts;
pub mod pgn;
pub mod play;
pub mod playout;
pub mod repetition_detector;
pub mod search;
pub mod settings;
pub mod setup;
pub mod stats;
pub mod uct;
pub mod utils;
