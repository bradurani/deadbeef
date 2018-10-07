extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_move;

mod helpers;

#[test]
fn promotion_to_queen_mate_white() {
    assert_move("8/p7/P7/6p1/4p2p/2pk4/5p2/2K5 b - - 1 44", "f2f1q");
}
