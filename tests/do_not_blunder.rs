#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::*;

mod helpers;

#[test]
fn do_not_sacrifice_knight() {
    assert_not_move(
        "rn1k1bnr/ppp1pppp/8/3N4/6b1/8/PPPP1P1P/R1B1KBNR w KQ - 2 7",
        "d5c7",
    );
}

#[test]
fn do_not_sacrifice_knight_2() {
    assert_not_move(
        "r1bk1b1r/ppp2ppp/2n1qn2/1N2p3/8/4PQ2/PP1P1PPP/R1B1KBNR w KQ - 8 8",
        "b5c7",
    );
}

#[test]
fn takes_knight_preventing_mate_in_1() {
    assert_contains_move(
        "r3r1k1/bppq1ppp/p2pbn2/4p3/1PPnP3/P1N2PP1/3BN2P/R2QKB1R w KQ - 0 14",
        vec!["e2d4", "e2c1"],
    );
}

#[test]
fn do_not_open_position_with_pawn_push() {
    assert_not_move(
        "r3r1k1/bppq1ppp/p1npbn2/4p3/1PP1P3/P1NP1PP1/3BN2P/R2QKB1R w KQ - 3 13",
        "d3d4",
    );
}

#[test]
fn do_not_allow_bishop_fork() {
    assert_move(
        "4rrk1/pp1b1ppp/q2p1n2/2p1p1N1/1nP1P3/1Q1P3P/PP1B1PP1/3RKB1R w K - 9 14",
        "b3c3",
    );
}

#[test]
fn do_not_sacrifice_bishop_on_c7() {
    assert_not_move(
        "2rqkbnr/1pp1pppp/p1n5/1N3b2/3PpB2/8/PPP2PPP/R2QKBNR w KQk - 0 7",
        "f4c7",
    );
}
