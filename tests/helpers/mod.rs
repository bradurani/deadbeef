#![allow(dead_code)]

extern crate deadbeef;
extern crate shakmaty;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::Chess;

const DEFAULT_ENSEMBLE_SIZE: usize = 4;
const DEFAULT_C: f32 = 0.5;
const DEFAULT_N_SAMPLES: isize = 20000;

pub fn assert_move(fen: &'static str, move_uci: &'static str) {
    assert_move_with_params(
        fen,
        move_uci,
        DEFAULT_ENSEMBLE_SIZE,
        DEFAULT_C,
        DEFAULT_N_SAMPLES,
    );
}

pub fn assert_move_in_n(fen: &'static str, move_uci: &'static str, n_samples: isize) {
    assert_move_with_params(fen, move_uci, DEFAULT_ENSEMBLE_SIZE, DEFAULT_C, n_samples);
}

fn assert_move_with_params(
    fen: &'static str,
    move_uci: &'static str,
    ensemble_size: usize,
    c: f32,
    n_samples: isize,
) {
    let setup: Fen = fen.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    let uci: Uci = move_uci.parse().unwrap();
    let m = uci.to_move(&position).unwrap();

    let best_child = play::find_best_move(
        &mut MCTS::new(1),
        TreeNode::new_root(&position, 0.5),
        &position,
        ensemble_size,
        0.,
        c,
        0.0,
        n_samples,
    )
    .unwrap();

    assert_eq!(m, best_child.action.unwrap())
}
