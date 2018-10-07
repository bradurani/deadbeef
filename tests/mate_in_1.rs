extern crate deadbeef;
extern crate shakmaty;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::Chess;

#[test]
fn queen_mate_white() {
    assert_move("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", "a7e7");
}

#[test]
fn queen_mate_black() {
    assert_move("4K3/q7/5k2/8/8/8/8/8 b - - 0 1", "a7e7");
}

#[test]
fn queen_capture_mate_black() {
    assert_move("1q6/8/5k2/4b3/8/8/PPP5/1K6 b - - 0 1", "b8b2");
}

#[test]
fn knight_mate_white() {
    assert_move("6rk/6pp/7N/8/3K4/8/8/8 w - - 0 1", "h6f7");
}

#[test]
fn discovered_checkmate_white(){
    assert_move("3rkb2/3q1pBp/4Np2/p7/Pp6/1P5P/2P2PP1/2QrRK2 w - - 0 1", "e6c7");
}

fn assert_move(fen: &'static str, move_uci: &'static str){
    let setup: Fen = fen.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    let uci: Uci = move_uci.parse().unwrap();
    let m = uci.to_move(&position).unwrap();

    let (action, _new_root) = play::make_move(
        &mut MCTS::new(),
        TreeNode::new_root(&position, 50.),
        &position,
        1,
        1000.0,
        0.5,
        100.0,
        )
        .unwrap();
    assert_eq!(m, action)
}
