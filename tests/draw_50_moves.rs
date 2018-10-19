extern crate deadbeef;
extern crate shakmaty;

use deadbeef::stats::*;
use helpers::*;

mod helpers;

#[test]
fn draw_50_move_rule_1() {
    let mut stats: RunStats = Default::default();
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 100 150", &mut stats);
    assert!(stats.nodes_created < 30);
}

#[test]
fn draw_50_move_rule_in_4() {
    let mut stats: RunStats = Default::default();
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 96 150", &mut stats);
    assert!(stats.nodes_created < 5000);
}

#[test]
fn draw_50_move_rule_in_5() {
    let mut stats: RunStats = Default::default();
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 95 150", &mut stats);
    assert!(stats.nodes_created < 25000);
}

#[test]
fn draw_50_move_rule_in_6() {
    let mut stats: RunStats = Default::default();
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 94 150", &mut stats);
    assert!(stats.nodes_created < 80000);
}

#[test]
fn black_captures_to_prevent_draw() {
    assert_mate_move("7k/7q/8/4p3/4P3/8/8/K7 b - - 99 150", "h7e4");
}
