#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use helpers::*;

mod helpers;

#[test]
fn queen_mate_white_in_1() {
    let stats = assert_mate_move("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", "a7e7");
    // assert!(stats.max_depth() == 1);
}

#[test]
fn queen_mate_black_in_1() {
    let stats = assert_mate_move("4K3/q7/5k2/8/8/8/8/8 b - - 0 1", "a7e7");
    // assert!(stats.max_depth() == 1);
}

#[test]
fn queen_capture_mate_black_in_1() {
    let stats = assert_mate_move("1q6/8/5k2/4b3/8/8/PPP5/1K6 b - - 0 1", "b8b2");
    // assert!(stats.max_depth() == 1);
}

#[test]
fn smothered_mate_white_in_1() {
    let stats = assert_mate_move("6rk/6pp/7N/8/3K4/8/8/8 w - - 0 1", "h6f7");
    // assert!(stats.max_depth() == 1);
}

#[test]
fn discovered_checkmate_white_in_1() {
    let stats = assert_mate_move(
        "3rkb2/3q1pBp/4Np2/p7/Pp6/1P5P/2P2PP1/2QrRK2 w - - 0 1",
        "e6c7",
    );
    // assert!(stats.max_depth() == 1);
}

#[test]
fn en_passant_mate_in_1() {
    let stats = assert_mate_move(
        "r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq f3 0 1",
        "e4f3",
    );
    // assert!(stats.max_depth() == 1);
}

// positions with more than 1 mate solution
#[test]
fn castle_mate_in_1() {
    // white long castle or kd2
    //  Edward Lasker–Sir George Thomas (London 1912)
    let stats = assert_contains_mate_move(
        "rn3r2/pbppq1p1/1p2pN2/8/3P2NP/6P1/PPP1BP1R/R3K1k1 w Q - 5 18",
        vec!["e1d2", "e1c1"],
    );
    // assert!(stats.max_depth() == 1);
}

#[test]
fn promotion_mate_white_in_1() {
    // can mate promoting to queen or rook
    let stats = assert_contains_mate_move(
        "8/p7/P7/6p1/4p2p/2pk4/5p2/2K5 b - - 1 44",
        vec!["f2f1q", "f2f1r"],
    );
    // assert!(stats.max_depth() == 1);
}

#[test]
fn queen_multi_mate_white_in_1() {
    let stats = assert_contains_mate_move(
        "4k3/1Q6/4K3/8/8/8/8/8 w - - 0 1",
        vec!["b7a8", "b7b8", "b7c8", "b7e7"],
    );
    // assert!(stats.max_depth() == 1);
}
