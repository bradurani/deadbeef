extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_move;

mod helpers;

#[test]
#[ignore]
fn queen_sacrifice_mate_in_4_white() {
    assert_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN2/6k1/3P2N1/3B4/PPP2PPP/R3K2R w KQ - 3 14",
        "h2h4",
    );
}