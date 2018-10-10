#![feature(duration_as_u128)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate rand;
extern crate shakmaty;
extern crate twox_hash;

pub mod game;
pub mod mcts;
pub mod pgn;
pub mod play;
pub mod playout;
pub mod tree_merge;
pub mod utils;
