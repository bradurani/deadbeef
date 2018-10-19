extern crate deadbeef;
extern crate shakmaty;

use helpers::assert_move;

mod helpers;

#[test]
#[ignore]
fn under_promotion_knight_endgame_white() {
    //must under promote to knight to prevent a stalemate
    assert_move("8/8/8/8/4k3/4p2r/4Kp2/6R1 b - - 1 67", "f2g1n");
}

#[test]
#[ignore]
fn under_promotion_rook_endgame_white() {
    //must under promote to rook to prevent a stalemate
    assert_move("8/6P1/7k/8/6K1/8/8/8 w - - 0 1", "g7g8r");
}

#[test]
#[ignore]
fn king_rook_endgame_white() {
    //must under promote to rook to prevent a stalemate
    assert_move("8/6P1/7k/8/6K1/8/8/8 w - - 0 1", "g8g5");
}

#[test]
fn rook_skewer() {
    assert_move("R7/P4k2/8/8/8/8/r7/5K2 w - - 0 1", "a8h8");
}
