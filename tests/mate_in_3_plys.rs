extern crate deadbeef;
extern crate log;
extern crate shakmaty;

use helpers::*;

mod helpers;

#[test]
fn queen_sacrifice_mate_in_3_plys() {
    assert_mate_move(
        "r2q1r2/pp2np2/1bp4p/3p2pk/1P1N2b1/2PB2B1/P5PP/R2QK2R w KQ - 0 1",
        "d1g4", //-> Kh5xg4 -> bd3e2#
    );
}

#[test]
fn queen_sacrifice_knight_mate_in_3_plys() {
    assert_mate_move(
        "2r2r1k/p1q3pp/8/3Q1p2/2N5/PP3N2/4n1P1/R1B2n1K b - - 0 1",
        "c7h2",
    );
}

#[test]
fn bishop_mate_in_3_plys() {
    assert_mate_move(
        "r4b1r/pppbkBpp/q1n3n1/5p2/2NPp3/1QP5/PP3PPP/RNB2RK1 w - - 0 1",
        "c1g5",
    );
}

// with multiple solutions
#[test]
fn queen_mate_in_3_plys() {
    //e8e2, e8e5 d3c3
    assert_contains_mate_move(
        "4q3/8/8/8/8/3k4/8/3K4 b - - 0 1",
        vec!["d3c3", "e8e2", "e8e5"],
    );
}

#[test]
fn rook_mate_in_3_plys() {
    assert_contains_mate_move(
        "8/2k5/K7/6r1/8/8/8/8 b - - 6 115",
        vec!["g5h5", "g5f5", "g5e5", "g5d5", "g5c5"],
    );
}

#[test]
fn back_row_mate_in_3_plys_white() {
    assert_contains_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN2/8/3P2NP/6P1/PPP1BPk1/R3K2R w KQ - 3 17",
        vec!["h1h2", "e1c1"],
    );
}

#[test]
fn king_back_row_mate_in_3_plys() {
    //  Edward Lasker–Sir George Thomas (London 1912)
    assert_contains_mate_move(
        //  Edward Lasker–Sir George Thomas (London 1912)
        "rn3r2/pbppq1p1/1p2pN2/8/3P2NP/3B1kP1/PPP2P2/R3K2R w KQ - 1 16",
        vec!["e1f1", "e1g1"],
    );
}
