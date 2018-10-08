extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_move;

mod helpers;

#[test]
#[ignore]
fn queen_sacrifice_mate_in_6_white() {
    assert_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1pk/1p2pb2/4N3/3PN3/3B4/PPP2PPP/R3K2R w KQ - 0 12",
        "e4f6",
    );
}
