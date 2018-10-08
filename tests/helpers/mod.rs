extern crate deadbeef;
extern crate shakmaty;

use deadbeef::mcts::{TreeNode, MCTS};
use deadbeef::play;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::Chess;

pub fn assert_move(fen: &'static str, move_uci: &'static str) {
    let setup: Fen = fen.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    let uci: Uci = move_uci.parse().unwrap();
    let m = uci.to_move(&position).unwrap();

    let (action, _new_root) = play::make_move(
        &mut MCTS::new(1),
        TreeNode::new_root(&position, 50.),
        &position,
        1,
        0.,
        0.5,
        0.0,
        5000,
    )
    .unwrap();
    assert_eq!(m, action)
}
