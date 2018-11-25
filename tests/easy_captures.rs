#[macro_use]
extern crate log;
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
fn black_does_not_blunder_queen() {
    assert_move("5k1q/5Npp/3PK3/8/8/8/8/8 b - - 0 1", "h8g8");
}

#[test]
#[ignore] //this one is tough
fn black_saves_its_queen() {
    assert_move("3kq1b1/3p1p2/8/8/8/8/4R3/3QK3 b - - 0 1", "e8f8");
}

#[test]
fn capture_bishop_and_save_queen() {
    assert_contains_move(
        "3qkbnr/p1Bppp1p/b1r3p1/8/3PP3/2N2N2/PPP2PPP/R2Q1RK1 b k - 0 9",
        vec!["c6c7", "d8c7"],
    );
}

#[test]
fn do_not_blunder_knight() {
    assert_contains_move(
        "rnbqkbnr/ppp1pppp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 3",
        vec!["c1f4", "b1c3"],
    );
}

#[test]
fn do_not_blunder_bishop() {
    assert_contains_move(
        "rnbqkbnr/pppp1ppp/4p3/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2",
        vec!["c1f4", "c1g5"],
    );
}
