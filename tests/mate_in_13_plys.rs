#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_mate_move;

mod helpers;

#[test]
#[ignore]
fn queen_sacrifice_mate_in_13_plys() {
    assert_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3rk1/pbppq1pp/1p2pb2/4N2Q/3PN3/3B4/PPP2PPP/R3K2R w KQ - 6 11",
        "h5h7",
    );
}
