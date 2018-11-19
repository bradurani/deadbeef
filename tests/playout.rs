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
fn saves_bishop() {
    let reward = reward_test("rnbqkbnr/pp3ppp/2p5/1B1pp3/8/P3P3/1PPP1PPP/RNBQK1NR w KQkq - 0 4");
    assert!(reward > -200);
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_white_to_play() {
    let reward = reward_test("7k/7r/8/8/8/8/R7/RK6 w - -");
    assert!(reward > 0);
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_black_to_play() {
    let reward = reward_test("7k/7r/8/8/8/8/R7/RK6 b - -");
    assert!(reward > 0);
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_white_to_play() {
    let reward = reward_test("rk6/r7/8/8/8/8/7R/7K w - -");
    assert!(reward < 0);
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_black_to_play() {
    let reward = reward_test("rk6/r7/8/8/8/8/7R/7K b - -");
    assert!(reward < 0);
}

fn black_mate_in_1() {
    let reward = reward_test("8/p7/P7/6p1/4p2p/2pk4/5p2/2K5 b - - 1 44");
    assert!(reward == MIN_REWARD);
}

#[test]
fn white_mate_in_1() {
    let reward = reward_test("rn3r2/pbppq1p1/1p2pN2/8/3P2NP/6P1/PPP1BP1R/R3K1k1 w Q - 5 18");
    assert!(reward == MAX_REWARD);
}

#[test]
fn white_mate_in_1_and_half() {
    let reward = reward_test("r4b1r/pppbkBpp/q1n3n1/5pB1/2NPp3/1QP5/PP3PPP/RN3RK1 b - - 1 1");
    assert!(reward == MAX_REWARD);
}

#[test]
fn white_mate_in_2() {
    let reward = reward_test("rn3r2/pbppq1p1/1p2pN2/8/3P2NP/6P1/PPP1BPk1/R3K2R w KQ - 3 17");
    assert!(reward == MAX_REWARD);
}

fn reward_test(fen_str: &'static str) -> Reward {
    setup();
    let position = parse_fen(fen_str);
    let mut stats: RunStats = Default::default();
    let settings = Settings::test_default();
    stats.start_timer();
    let reward = playout(position, &mut stats, &settings);
    stats.stop_timer();
    println!("reward is {}", reward);
    println!("{}", stats);
    reward
}
