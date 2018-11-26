#![allow(dead_code)]
#![allow(unused_variables)]
extern crate log;
// extern crate deadbeef;
extern crate shakmaty;

use self::log::*;
use self::shakmaty::*;
use deadbeef::engine::*;
use deadbeef::game::*;
use deadbeef::logger;
use deadbeef::search_strategy::*;
use deadbeef::settings::*;
use deadbeef::setup::*;
use deadbeef::stats::*;
use deadbeef::utils::*;
use std::time::Duration;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        logger::init();
    });
}

pub fn assert_move(fen_str: &'static str, uci_str: &'static str) -> RunStats {
    let settings = Settings::test_default();
    run_move_test(fen_str, vec![uci_str], vec![], &settings, false)
}

pub fn assert_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) -> RunStats {
    let settings = Settings::test_default();
    run_move_test(fen_str, uci_strs, vec![], &settings, false)
}

pub fn assert_mate_move(fen_str: &'static str, uci_str: &'static str) -> RunStats {
    let settings = Settings::test_default();
    run_move_test(fen_str, vec![uci_str], vec![], &settings, true)
}

pub fn assert_contains_mate_move(fen_str: &'static str, uci_strs: Vec<&'static str>) -> RunStats {
    let settings = Settings::test_default();
    run_move_test(fen_str, uci_strs, vec![], &settings, true)
}

pub fn assert_not_move(fen_str: &'static str, uci_str: &'static str) -> RunStats {
    let settings = Settings::test_default();
    run_not_move_test(fen_str, vec![uci_str], vec![], &settings, false)
}

pub fn assert_not_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) -> RunStats {
    let settings = Settings::test_default();
    run_not_move_test(fen_str, uci_strs, vec![], &settings, false)
}

pub fn assert_draw(fen_str: &'static str) -> RunStats {
    let (minimax, stats) = run_minimax_test(fen_str, &Settings::test_default());
    assert_eq!(minimax, 0);
    stats
}

pub fn assert_not_move_with_repetitions(
    fen_str: &'static str,
    uci_str: &'static str,
    repetitions: Vec<&'static str>,
) -> RunStats {
    let settings = Settings::test_default();
    run_not_move_test(fen_str, vec![uci_str], repetitions, &settings, false)
}

pub fn run_challenge_suite(filename: &'static str, times: &Vec<u64>) -> u16 {
    let contents = file_to_string(filename);
    contents.lines().fold(0, |mut score, line| {
        let tokens: Vec<&str> = line.splitn(2, " bm ").collect();
        let fen = tokens[0];
        let more_tokens: Vec<&str> = tokens[1].splitn(2, "; id ").collect();
        let sans: Vec<&str> = more_tokens[0].split(" ").collect();
        let mut engine = setup_engine(&fen, &Settings::test_default());
        let expected_actions: Vec<Move> = sans
            .iter()
            .map(|s| parse_san(s, &engine.position()))
            .collect();
        let id = more_tokens[1];
        let display_expected: Vec<String> =
            expected_actions.iter().map(|a| format!("{}", a)).collect();
        for time in times {
            let search_type = SearchType::Time(Duration::from_millis(*time));
            let engine_action = engine.test_search(&search_type);
            info!(
                "\n{} expecting {}. Found {}",
                id,
                display_expected.join(" "),
                engine_action
            );
            if expected_actions.contains(&engine_action) {
                score += 1
            }
        }
        score
    })
}

fn run_not_move_test(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    repetitions: Vec<&'static str>,
    settings: &Settings,
    assert_mate: bool,
) -> RunStats {
    run_move_test_and_assert(fen_str, uci_strs, repetitions, settings, assert_mate, false)
}

fn run_move_test(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    repetitions: Vec<&'static str>,
    settings: &Settings,
    assert_mate: bool,
) -> RunStats {
    run_move_test_and_assert(fen_str, uci_strs, vec![], settings, assert_mate, true)
}

fn run_move_test_and_assert(
    fen_str: &'static str,
    uci_strs: Vec<&'static str>,
    repetitions: Vec<&'static str>,
    settings: &Settings,
    assert_mate: bool,
    assert_contains_move: bool,
) -> RunStats {
    let mut engine = setup_engine(fen_str, settings);
    engine.record_test_repetitions(repetitions);
    let engine_move = engine
        .make_engine_move()
        .expect("could not make engine move");
    let expected_moves = expected_moves(&engine, uci_strs);

    let contains_move = expected_moves.contains(&engine_move);
    if contains_move ^ assert_contains_move {
        error!("FAIL! played {} in:", engine_move);
        engine.print_tree().expect("could not print tree");
        panic!(
            "{} {} found in {}",
            engine_move,
            if assert_contains_move { "not" } else { "" },
            move_list_string(expected_moves)
        );
    }
    if assert_mate {
        assert!(engine.is_decisive());
    }
    engine.game_stats
}

fn setup_engine(fen_str: &str, settings: &Settings) -> Engine {
    setup();
    let mut engine = Engine::new(settings.clone());
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

fn run_minimax_test(fen_str: &'static str, settings: &Settings) -> (Reward, RunStats) {
    let mut engine = setup_engine(fen_str, settings);
    engine.search_with_settings().unwrap();
    (engine.minimax(), engine.game_stats)
}
