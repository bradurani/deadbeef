#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_mate_move;

mod helpers;

#[test]
fn mate_in_5_plys_white() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN2/8/3P1kNP/3B4/PPP2PP1/R3K2R w KQ - 1 15",
        "g2g3",
    );
}

#[test]
fn smothered_mate_in_5_plys() {
    assert_mate_move("3r3k/6pp/3N4/3Q4/8/8/6K1/8 w - - 1 1", "d6f7");
}
