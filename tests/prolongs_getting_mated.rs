#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_mate_move;

mod helpers;

#[test]
fn checks_to_prolong_mate() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN2/8/3P2NP/6P1/PPPKBPk1/R6R b - - 4 17",
        "e7b4",
    );
}

#[test]
fn checks_to_prolong_mate_2() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbpp2p1/1p2pN2/8/1q1P2NP/4K1P1/PPP1BPk1/R6R b - - 6 18",
        "b4d4",
    );
}

#[test]
fn checks_to_prolong_mate_3() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbpp2p1/1p2pN2/8/3K2NP/6P1/PPP1BPk1/R6R b - - 0 19",
        "e6e5",
    );
}
