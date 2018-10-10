extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_move;

mod helpers;

#[test]
#[ignore]
fn queen_sacrifice_mate_in_5_white() {
    assert_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3rk1/pbppq1pp/1p2pb2/4N2Q/3PN3/3B4/PPP2PPP/R3K2R w KQ - 6 11",
        "e5g4",
    );
}
