extern crate shakmaty;
extern crate deadbeef;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::Chess;

#[test]
fn queen_mate_white() {
    let fen = "4k3/Q7/5K2/8/8/8/8/8 w - - 0 1";
    let setup: Fen = fen.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    let uci: Uci = "a7e7".parse().unwrap();
    let m = uci.to_move(&position).unwrap();

    let (action, _new_root) = play::make_move(&mut MCTS::new(), TreeNode::new_root(&position, 50.), &position, 1, 1000.0, 0.5, 100.0).unwrap();
    assert_eq!(m, action);
}

#[test]
fn queen_mate_black() {
    let fen = "4K3/q7/5k2/8/8/8/8/8 b - - 0 1";
    let setup: Fen = fen.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    let uci: Uci = "a7e7".parse().unwrap();
    let m = uci.to_move(&position).unwrap();

    let (action, _new_root) = play::make_move(&mut MCTS::new(), TreeNode::new_root(&position, 50.), &position, 1, 1000.0, 0.5, 100.0).unwrap();
    assert_eq!(m, action);
}
