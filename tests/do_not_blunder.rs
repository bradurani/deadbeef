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
        "d5c7",
    )
}
