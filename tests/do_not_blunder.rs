#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::*;

mod helpers;
#[test]
fn do_not_blunder_knight() {
    assert_not_move(
        "rnbqkbnr/ppp1pppp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 3",
        "g1f3",
    );
}

#[test]
fn do_not_blunder_bishop() {
    assert_not_move(
        "rnbqkbnr/pppp1ppp/4p3/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2",
        "c1h6",
    );
}

#[test]
fn do_not_sacrifice_knight() {
    assert_not_move(
        "rn1k1bnr/ppp1pppp/8/3N4/6b1/8/PPPP1P1P/R1B1KBNR w KQ - 2 7",
        "d5c7",
    );
}

#[test]
fn do_not_sacrifice_knight_2() {
    assert_not_move(
        "r1bk1b1r/ppp2ppp/2n1qn2/1N2p3/8/4PQ2/PP1P1PPP/R1B1KBNR w KQ - 8 8",
        "b5c7",
    );
}

#[test]
fn takes_knight_preventing_mate_in_1() {
    assert_contains_move(
        "r3r1k1/bppq1ppp/p2pbn2/4p3/1PPnP3/P1N2PP1/3BN2P/R2QKB1R w KQ - 0 14",
        vec!["e2d4", "e2c1", "e2g1"],
    );
}

#[test]
fn do_not_open_position_with_pawn_push() {
    assert_not_move(
        "r3r1k1/bppq1ppp/p1npbn2/4p3/1PP1P3/P1NP1PP1/3BN2P/R2QKB1R w KQ - 3 13",
        "d3d4",
    );
}

#[test]
fn do_not_allow_bishop_fork() {
    assert_move(
        "4rrk1/pp1b1ppp/q2p1n2/2p1p1N1/1nP1P3/1Q1P3P/PP1B1PP1/3RKB1R w K - 9 14",
        "b3c3",
    );
}

#[test]
fn do_not_sacrifice_bishop_on_c7() {
    assert_not_move(
        "2rqkbnr/1pp1pppp/p1n5/1N3b2/3PpB2/8/PPP2PPP/R2QKBNR w KQk - 0 7",
        "f4c7",
    );
}

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
// #[ignore] //this one is tough
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
fn does_not_hang_queen() {
    assert_not_move(
        "1r2r1k1/2pb2p1/p1n1p2p/5p2/Q2P4/1q1BPN1P/1P3PP1/2RR1K2 w - - 6 24",
        "f1e2",
    );
}
