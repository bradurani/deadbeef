extern crate deadbeef;
extern crate shakmaty;

use deadbeef::stats::*;
use helpers::*;

mod helpers;

#[test]
fn draw_threefold() {
    let mut stats: RunStats = Default::default();
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 100 150", &mut stats);
    assert!(stats.nodes_created < 30);
}

fn philidor() {
    let mut stats: RunStats = Default::default();
    assert_draw("4k3/8/4K3/4P3/8/8/r7/7R b - - 0 1", &mut stats);
    assert!(stats.nodes_created < 30);
}

fn force_threefold_with_queen() {
    let mut stats: RunStats = Default::default();
    assert_draw("q4r1k/5p2/8/8/8/8/8/2Q3K1 w - - 0 1", &mut stats);
    assert!(stats.nodes_created < 30);
}
