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
fn run_move_test(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    settings: Settings,
    assert_mate: bool,
) {
    setup();
    let mut engine = Engine::new(settings);
    engine.set_board(fen_str);
    let engine_move = engine.make_engine_move();

    let expected_moves: Vec<Move> = uci_strs
        .iter()
        .map(|u| parse_uci(u, &engine.previous_position))
        .collect();

    if assert_mate {
        assert!(engine.state.is_decisive());
    }
    if expected_moves.contains(&engine_move) {
        assert!(true);
    } else {
        let moves_str: Vec<String> = expected_moves.iter().map(|m| m.to_string()).collect();
        panic!("{} not found in {}", engine_move, moves_str.join(" ,"));
    }
}
