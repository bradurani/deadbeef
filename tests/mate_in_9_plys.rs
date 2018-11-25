#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_mate_move;

mod helpers;

#[test]
#[ignore]
fn mate_in_9_plys_white() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN1k/4N3/3P4/3B4/PPP2PPP/R3K2R w KQ - 1 13",
        "e5g4",
    );
}
