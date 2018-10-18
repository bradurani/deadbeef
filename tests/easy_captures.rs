extern crate deadbeef;
extern crate shakmaty;

use helpers::*;

mod helpers;

#[test]
fn makes_winning_queen_capture() {
    assert_move("8/2k5/8/5q2/8/5Q2/8/5K2 w - - 0 1", "f3f5");
}

#[test]
fn makes_winning_pawn_capture() {
    assert_move("8/4k3/8/8/8/8/2KR1p2/8 w - - 0 1", "d2f2");
}

#[test]
fn black_saves_its_queen() {
    assert_move("3kq1b1/3p1p2/8/8/8/8/4R3/3QK3 b - - 0 1", "e8f8");
}
