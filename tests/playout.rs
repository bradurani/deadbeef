extern crate deadbeef;
extern crate shakmaty;

use deadbeef::playout::*;
use deadbeef::settings::*;
use deadbeef::setup::*;
use deadbeef::stats::*;

#[test]
fn saves_bishop() {
    let position = parse_fen("rnbqkbnr/pp3ppp/2p5/1B1pp3/8/P3P3/1PPP1PPP/RNBQK1NR w KQkq - 0 4");
    let mut stats: RunStats = Default::default();
    let settings = Settings::test_default();
    let reward = playout(position, &mut stats, &settings);
    assert!(reward > 0);
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_white_to_play() {
    let position = parse_fen("7k/7r/8/8/8/8/R7/RK6 w - -");
    let mut stats: RunStats = Default::default();
    let settings = Settings::test_default();
    let reward = playout(position, &mut stats, &settings);
    println!("reward is {}", reward);
    assert!(reward > 0);
}

#[test]
fn is_positive_in_2_rook_vs_1_rook_endgame_for_white_black_to_play() {
    let position = parse_fen("7k/7r/8/8/8/8/R7/RK6 b - -");
    let mut stats: RunStats = Default::default();
    let settings = Settings::test_default();
    let reward = playout(position, &mut stats, &settings);
    println!("reward is {}", reward);
    assert!(reward > 0);
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_white_to_play() {
    let position = parse_fen("rk6/r7/8/8/8/8/7R/7K w - -");
    let mut stats: RunStats = Default::default();
    let settings = Settings::test_default();
    let reward = playout(position, &mut stats, &settings);
    println!("reward is {}", reward);
    assert!(reward < 0);
}

#[test]
fn is_negative_in_2_rook_vs_1_rook_endgame_for_black_black_to_play() {
    let position = parse_fen("rk6/r7/8/8/8/8/7R/7K b - -");
    let mut stats: RunStats = Default::default();
    let settings = Settings::test_default();
    let reward = playout(position, &mut stats, &settings);
    println!("reward is {}", reward);
    assert!(reward < 0);
}
