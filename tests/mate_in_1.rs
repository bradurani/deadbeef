extern crate shakmaty;
extern crate deadbeef;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::Chess;

#[test]
fn queen_mate() {
    let fen = "3k4/Q7/4K3/8/8/8/8/8 w - - 0 1";
    let setup: Fen = fen.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    let uci: Uci = "a7d7".parse().unwrap();
    let m = uci.to_move(&position).unwrap();

    let (action, _new_root) = play::make_move(&mut MCTS::new(), TreeNode::new_root(&position, 50.), &position, 1, 1000.0, 0.5, 100.0).unwrap();
    assert_eq!(m, action);
}
