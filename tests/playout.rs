#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use deadbeef::game::*;
use deadbeef::playout::*;
use deadbeef::settings::*;
use deadbeef::setup::*;
use deadbeef::stats::*;
use helpers::*;

mod helpers;

#[test]
fn mated_white() {
    for depth in 0..4 {
        let reward = reward_test("3k4/3Q4/3K4/8/8/8/8/8 b - -", depth);
        assert!(reward == MAX_REWARD);
    }
}

#[test]
fn mated_black() {
    for depth in 0..4 {
        let reward = reward_test("8/8/8/8/8/3k4/3q4/3K4 w - -", depth);
        assert!(reward == MIN_REWARD);
    }
}

#[test]
fn mate_in_1_white() {
    for depth in 1..4 {
        let reward = reward_test("4k3/8/4K3/8/8/8/8/7R w - -", depth);
        assert!(reward == MAX_REWARD);
    }
}

#[test]
fn mate_in_1_black() {
    for depth in 1..4 {
        let reward = reward_test("5r2/8/8/8/8/3k4/8/3K4 b - -", depth);
        assert!(reward == MIN_REWARD);
    }
}

#[test]
fn mate_in_1_and_half_white() {
    for depth in 2..4 {
        let reward = reward_test("3R4/8/8/8/8/1K6/8/k7 b - -", depth);
        assert!(reward == MAX_REWARD);
    }
}

#[test]
fn mate_in_1_and_half_black() {
    for depth in 2..4 {
        let reward = reward_test("K7/8/1k6/8/8/8/8/3r4 w - -", depth);
        assert!(reward == MIN_REWARD);
    }
}

fn mate_in_2_white() {
    for depth in 3..5 {
        let reward = reward_test("k7/4R3/1K6/8/8/8/8/8 w - -", depth);
        assert!(reward == MAX_REWARD);
    }
}

#[test]
fn mate_in_2_white_complex() {
    for depth in 3..5 {
        let reward = reward_test(
            "rn3r2/pbppq1p1/1p2pN2/8/3P2NP/6P1/PPP1BPk1/R3K2R w KQ -",
            depth,
        );
        assert!(reward == MAX_REWARD);
    }
}

#[test]
fn mate_in_2_black() {
    for depth in 3..5 {
        let reward = reward_test("8/8/8/8/8/1k6/4r3/K7 b - -", depth);
        assert!(reward == MIN_REWARD);
    }
}

#[test]
fn mate_in_2_and_half_white() {
    for depth in 4..5 {
        let reward = reward_test("1k6/7R/2K5/8/8/8/8/8 b - -", depth);
        assert!(reward == MAX_REWARD);
    }
}

#[test]
fn mate_in_2_and_half_black() {
    for depth in 4..5 {
        let reward = reward_test("8/8/8/8/8/2k5/6r1/1K6 w - -", depth);
        assert!(reward == MIN_REWARD);
    }
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_white_to_play() {
    for depth in 0..4 {
        let reward = reward_test("7k/7r/8/8/8/8/R7/RK6 w - -", depth);
        assert!(reward > 0);
    }
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_black_to_play() {
    for depth in 0..4 {
        let reward = reward_test("rk6/r7/8/8/8/8/7R/7K b - -", depth);
        assert!(reward < 0);
    }
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_black_to_play() {
    // at depth 4 this fails due to odd / even and needs q search
    for depth in 0..3 {
        let reward = reward_test("7k/7r/8/8/8/8/R7/RK6 b - -", depth);
        assert!(reward > 0);
    }
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_white_to_play() {
    // at depth 4 this fails due to odd / even and needs q search
    for depth in 0..3 {
        let reward = reward_test("rk6/r7/8/8/8/8/7R/7K w - -", depth);
        assert!(reward < 0);
    }
}

//  WITH Q SEARCH

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_white_to_play_deep_with_q() {
    for depth in 0..5 {
        let reward = reward_test_with_q("7k/7r/8/8/8/8/R7/RK6 w - -", depth);
        assert!(reward > 0);
    }
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_black_to_play_deep_with_q() {
    for depth in 0..5 {
        let reward = reward_test_with_q("rk6/r7/8/8/8/8/7R/7K b - -", depth);
        assert!(reward < 0);
    }
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_black_to_play_deep_with_q() {
    for depth in 0..5 {
        let reward = reward_test_with_q("7k/7r/8/8/8/8/R7/RK6 b - -", depth);
        assert!(reward > 0);
    }
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_white_to_play_deep_with_q() {
    for depth in 0..5 {
        let reward = reward_test_with_q("rk6/r7/8/8/8/8/7R/7K w - -", depth);
        assert!(reward < 0);
    }
}

// RANDOM BLUNDER TESTS

#[test]
fn saves_bishop() {
    for depth in 0..5 {
        let reward = reward_test(
            "rnbqkbnr/pp3ppp/2p5/1B1pp3/8/P3P3/1PPP1PPP/RNBQK1NR w KQkq - 0 4",
            depth,
        );
        assert!(reward > -200);
    }
}

// TEST HELPERS

fn reward_test_with_q(fen_str: &'static str, depth: isize) -> Reward {
    playout_test(fen_str, depth, true)
}
fn reward_test(fen_str: &'static str, depth: isize) -> Reward {
    playout_test(fen_str, depth, false)
}

fn playout_test(fen_str: &'static str, depth: isize, q_search: bool) -> Reward {
    setup();
    let position = parse_fen(fen_str);
    let mut stats: RunStats = Default::default();
    let settings = Settings::playout_test(depth, q_search);
    stats.start_timer();
    println!("DEPTH: {}, Q SEARCH: {}", depth, q_search);
    let reward = playout(position, &mut stats, &settings);
    stats.stop_timer();
    println!("reward is {}", reward);
    println!("{}", stats);
    reward
}
