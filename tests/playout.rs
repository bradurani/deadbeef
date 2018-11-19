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
