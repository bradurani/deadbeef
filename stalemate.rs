extern crate deadbeef;
extern crate shakmaty;

use deadbeef::stats::*;
use helpers::*;

mod helpers;

#[test]
fn stalemate_rook_king() {
    let mut stats: RunStats = Default::default();
    assert_draw("3k4/7r/3KP3/8/8/8/8/R7 b - - 0 1", &mut stats);
    assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_down_a_pawn() {
    let mut stats: RunStats = Default::default();
    assert_draw("7k/5Kp1/5p1p/5P1P/8/8/8/8 w - - 0 1", &mut stats);
    assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_rook_sacrifice() {
    let mut stats: RunStats = Default::default();
    assert_draw("1q5k/5Rp1/6K1/4N2P/8/8/8/8 w - - 0 1", &mut stats);
    assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_queen_trade() {
    let mut stats: RunStats = Default::default();
    assert_draw("8/8/6Q1/2p5/1pk3nb/5q2/5R2/r1N1K3 w - - 0 1", &mut stats);
    assert!(stats.nodes_created < 30);
}
