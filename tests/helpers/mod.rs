#![allow(dead_code)]

extern crate deadbeef;
extern crate shakmaty;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use deadbeef::settings::*;
use deadbeef::stats::*;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::{Chess, Move};

pub fn assert_move(fen: &'static str, move_uci: &'static str) {
    let mut test_run_stats: RunStats = Default::default();
    let settings = Settings::test_default();
    assert_move_with_settings(fen, move_uci, &mut test_run_stats, settings)
}

pub fn assert_move_with_settings(
    fen_str: &'static str,
    uci_str: &'static str,
    test_run_stats: &mut RunStats,
    settings: Settings,
) {
    let position = parse_fen(fen_str);
    let m = parse_uci(uci_str, &position);

    let best_child = play::find_best_move(
        &mut MCTS::new(&settings),
        TreeNode::new_root(&position, settings.starting_iterations_per_ms),
        &position,
        test_run_stats,
        &settings,
    )
    .unwrap();

    assert_eq!(m, best_child.action.unwrap())
}

fn parse_fen(fen_str: &'static str) -> Chess {
    let setup: Fen = fen_str.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    position
}

fn parse_uci(uci_str: &'static str, position: &Chess) -> Move {
    let uci: Uci = uci_str.parse().unwrap();
    let m = uci.to_move(position).unwrap();
    m
}
