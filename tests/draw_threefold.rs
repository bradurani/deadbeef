#[macro_use]
extern crate log;
extern crate deadbeef;
extern crate shakmaty;

use deadbeef::stats::*;
use helpers::*;

mod helpers;

#[test]
#[ignore]
fn philidor() {
    // let mut stats: RunStats = Default::default();
    // assert_draw("4k3/8/4K3/4P3/8/8/r7/7R b - - 0 1", &mut stats);
    // assert!(stats.nodes_created < 30);
}

#[test]
fn force_threefold_with_queen() {
    let stats = assert_move("q4r1k/5p2/8/8/8/8/8/2Q3K1 w - - 0 1", "c1h6");
    // assert!(stats.nodes_created < 30);
}

#[test]
fn does_not_draw_when_ahead() {
    assert_move(
        "3rr1k1/1bp1n2p/p2qP1p1/8/R7/5N1P/BPQ2PP1/4R1K1 w - - 9 29",
        "a4a5",
    ); // ensures test is valid
    assert_not_move_with_repetitions(
        "3rr1k1/1bp1n2p/p2qP1p1/8/R7/5N1P/BPQ2PP1/4R1K1 w - - 9 29",
        "a4a5",
        vec![
            "3rr1k1/1bp1n2p/p2qP1p1/R7/8/5N1P/BPQ2PP1/4R1K1 b - - 10 29",
            "3rr1k1/1bp1n2p/p2qP1p1/R7/8/5N1P/BPQ2PP1/4R1K1 b - - 10 29",
        ],
    );
}
