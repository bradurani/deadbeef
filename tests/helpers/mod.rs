#![allow(dead_code)]

extern crate deadbeef;
extern crate shakmaty;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use deadbeef::settings::*;
use deadbeef::setup::*;
use deadbeef::stats::*;
use shakmaty::{uci::Uci, Move, Outcome};
use std::convert::From;

pub fn assert_move(fen_str: &'static str, uci_str: &'static str) {
    assert_contains_move(fen_str, vec![uci_str]);
}

pub fn assert_mate_move(fen_str: &'static str, uci_str: &'static str) {
    assert_contains_mate_move(fen_str, vec![uci_str]);
}

pub fn assert_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
    let mut test_run_stats: RunStats = Default::default();
    let settings = Settings::test_default();
    assert_contains_move_with_settings(fen_str, uci_strs, &mut test_run_stats, &settings, false)
}

pub fn assert_contains_mate_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
    let mut test_run_stats: RunStats = Default::default();
    let settings = Settings::test_default();
    assert_contains_move_with_settings(fen_str, uci_strs, &mut test_run_stats, &settings, true)
}

pub fn assert_contains_mate_move_with_settings(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    test_run_stats: &mut RunStats,
    settings: &Settings,
) {
    assert_contains_move_with_settings(fen_str, uci_strs, test_run_stats, settings, true);
}

pub fn assert_contains_move_with_settings(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    test_run_stats: &mut RunStats,
    settings: &Settings,
    assert_mate: bool,
) {
    let position = parse_fen(fen_str);
    let moves: Vec<Move> = uci_strs.iter().map(|u| parse_uci(u, &position)).collect();

    let best_child = play::find_best_move(
        &mut MCTS::new(&settings),
        TreeNode::new_root(&position, settings.starting_iterations_per_ms),
        &position,
        test_run_stats,
        &settings,
    )
    .unwrap();
    let best_action = best_child.action.unwrap();
    if moves.contains(&best_action) {
        if assert_mate {
            let is_definitive = match best_child.outcome {
                Some(Outcome::Decisive { winner }) => true,
                _ => false,
            };
            assert!(is_definitive);
        } else {
            assert!(true);
        }
    } else {
        let moves_str: Vec<String> = moves.iter().map(|m| format!("{}", m)).collect();

        panic!("{} not found in {}", best_action, moves_str.join(" ,"));
    }
}
