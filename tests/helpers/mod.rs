#![allow(dead_code)]
#![allow(unused_variables)]
extern crate deadbeef;
extern crate shakmaty;

use deadbeef::engine::*;
use deadbeef::logger;
use deadbeef::settings::*;
use deadbeef::setup::*;
use deadbeef::stats::*;
use shakmaty::*;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        logger::init();
    });
}

pub fn assert_move(fen_str: &'static str, uci_str: &'static str) {
    let settings = Settings::test_default();
    run_move_test(fen_str, vec![uci_str], settings, false);
}

pub fn assert_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
    let settings = Settings::test_default();
    run_move_test(fen_str, uci_strs, settings, false);
}

pub fn assert_mate_move(fen_str: &'static str, uci_str: &'static str) {
    let settings = Settings::test_default();
    run_move_test(fen_str, vec![uci_str], settings, true);
}

pub fn assert_contains_mate_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
    let settings = Settings::test_default();
    run_move_test(fen_str, uci_strs, settings, true);
}

pub fn assert_not_move(fen_str: &'static str, uci_str: &'static str) {
    let settings = Settings::test_default();
    run_not_move_test(fen_str, vec![uci_str], settings);
}

pub fn assert_not_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
    let settings = Settings::test_default();
    run_not_move_test(fen_str, uci_strs, settings);
}
//
// pub fn assert_mate_move(fen_str: &'static str, uci_str: &'static str) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_mate_default();
//     assert_contains_move_with_settings(
//         fen_str,
//         vec![uci_str],
//         &mut test_run_stats,
//         &settings,
//         true,
//     );
// }
//
// pub fn assert_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_default();
//     assert_contains_move_with_settings(fen_str, uci_strs, &mut test_run_stats, &settings, false)
// }
//
// pub fn assert_contains_mate_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_mate_default();
//     assert_contains_move_with_settings(fen_str, uci_strs, &mut test_run_stats, &settings, true)
// }
//
// pub fn assert_draw(fen_str: &'static str, stats: &mut RunStats) {
//     let settings = Settings::test_mate_default();
//     let position = parse_fen(fen_str);
//     println!("halfmove clock: {}", position.halfmoves());
//     let root = TreeNode::new_root(&position, 100.);
//
//     println!("{}", settings);
//
//     let new_root = play::find_best_move(root, &position, stats, &settings);
//
//     assert!(new_root.map_or(false, |o| o.is_draw()));
// }
//
fn run_not_move_test(fen_str: &'static str, uci_strs: Vec<&'static str>, settings: Settings) {
    let mut engine = setup_engine(fen_str, settings);
    let engine_move = engine
        .make_engine_move()
        .expect("could not make engine move");
    let expected_moves = expected_moves(&engine, uci_strs);

    if expected_moves.contains(&engine_move) {
        panic!(
            "{} found in {}",
            engine_move,
            move_list_string(expected_moves)
        );
    } else {
        assert!(true);
    }
}

fn run_move_test(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    settings: Settings,
    assert_mate: bool,
) {
    let mut engine = setup_engine(fen_str, settings);
    let engine_move = engine
        .make_engine_move()
        .expect("could not make engine move");
    let expected_moves = expected_moves(&engine, uci_strs);

    if assert_mate {
        assert!(engine.is_decisive());
    }

    if expected_moves.contains(&engine_move) {
        assert!(true);
    } else {
        panic!(
            "{} not found in {}",
            engine_move,
            move_list_string(expected_moves)
        );
    }
}

fn setup_engine(fen_str: &str, settings: Settings) -> Engine {
    setup();
    let mut engine = Engine::new(settings);
    engine.set_board(fen_str).unwrap();
    engine
}

fn expected_moves(engine: &Engine, uci_strs: Vec<&'static str>) -> Vec<Move> {
    uci_strs
        .iter()
        .map(|u| parse_uci(u, &engine.previous_position))
        .collect()
}

fn move_list_string(moves: Vec<Move>) -> String {
    let move_strings: Vec<String> = moves.iter().map(|m| m.to_string()).collect();
    move_strings.join(", ")
}
