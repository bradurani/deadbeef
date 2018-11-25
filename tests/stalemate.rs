extern crate deadbeef;
extern crate log;
extern crate shakmaty;

use deadbeef::stats::*;
use helpers::*;

mod helpers;

#[test]
fn stalemate_rook_king() {
    assert_move("3k4/7r/3KP3/8/8/8/8/R7 b - - 0 1", "h7d7");
    // assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_down_a_pawn() {
    // kpp vs kpp white draw with Kg6, kf8, ke6
    // Kg6 is immediate draw, but Kf8 and Ke6 have positive q because black could make a mistake
    // Kf8 is probably best here because it gives white the most opportunities to blow it.
    assert_move("7k/5Kp1/5p1p/5P1P/8/8/8/8 w - - 0 1", "f7f8");
    // assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_rook_sacrifice() {
    // very complex stalemate position with many lines
    // requires deep search
    assert_move("1q5k/5Rp1/6K1/4N2P/8/8/8/8 w - - 0 1", "f7f8");
    // assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_queen_trade() {
    // stalemate queen sacrfice with 2 pieces pinned
    // tests that we use the draw move instead of largest n if a draw is possible
    assert_move("8/8/6Q1/2p5/1pk3nb/5q2/5R2/r1N1K3 w - - 0 1", "g6d3");
    // assert!(stats.nodes_created < 30);
}

#[test]
fn stalemate_sacrifice_rook_skewer() {
    // black rook can sacrifice or skewer queen
    assert_move("3r3k/4KQ2/8/8/8/8/8/8 b - - 51 87", "d8d7");
}

// ""
