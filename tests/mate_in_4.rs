#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_mate_move;

mod helpers;

#[test]
fn mate_in_4_white() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN2/6k1/3P2N1/3B4/PPP2PPP/R3K2R w KQ - 3 14",
        "h2h4",
    );
}

#[test]
fn smothered_mate_in_4() {
    assert_mate_move("4r2k/6pp/8/3Q2NK/8/8/8/8 w - - 1 1", "g5f7");
}
