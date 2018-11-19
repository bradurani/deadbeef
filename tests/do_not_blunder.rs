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
    )
}

#[test]
fn takes_knight_preventing_mate_in_1() {
    assert_contains_move(
        "r3r1k1/bppq1ppp/p2pbn2/4p3/1PPnP3/P1N2PP1/3BN2P/R2QKB1R w KQ - 0 14",
        vec!["e2d4", "e2c1"],
    )
}
