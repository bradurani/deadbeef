extern crate deadbeef;
extern crate log;
extern crate shakmaty;

use deadbeef::stats::*;
use helpers::*;

mod helpers;

// defunct test because we handle leaf node roots by not allowing searching in engine.rs
// #[test]
// fn draw_50_move_rule_0_ply() {
//     assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 100 150");
// }

#[test]
fn draw_50_move_rule_1_ply() {
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 99 150");
}

#[test]
fn draw_50_move_rule_2_ply() {
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 98 150");
}

#[test]
fn draw_50_move_rule_3_ply() {
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 98 150");
}

#[test]
fn draw_50_move_rule_in_4_plys() {
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 96 150");
}

#[test]
fn draw_50_move_rule_in_5_plys() {
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 95 150");
}

#[test]
fn draw_50_move_rule_in_6_plys() {
    assert_draw("7k/8/8/4p3/4P3/8/8/K7 w - - 94 150");
}

#[test]
fn black_captures_to_prevent_draw() {
    assert_move("7k/7q/8/4p3/4P3/8/8/K7 b - - 99 150", "h7e4");
}

#[test]
fn black_captures_in_2_to_prevent_draw() {
    assert_move("8/7r/8/4p3/1k2P3/8/8/1K6 b - - 98 150", "h7h4");
}

#[test]
fn black_move_pawn_to_prevent_draw() {
    assert_move("8/4p2r/8/8/2k5/8/8/1K6 b - - 100 150", "e7e5");
}
