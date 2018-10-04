extern crate shakmaty;

use shakmaty::{Move, Square, Role};
use mcts::TreeNode;

pub fn merge_trees(roots: Vec<TreeNode>) -> TreeNode {
    TreeNode::starting()
}

#[cfg(test)]
mod tests {

    use shakmaty::{Move, Square, Role, Color};
    use mcts::{TreeNode, NodeState};
    use tree_merge::{merge_trees};

    fn norm(role: char, from: &'static str, to: &'static str) -> Option<Move> {
        let m = Move::Normal {
            role: Role::from_char(role).unwrap(),
            from: Square::from_ascii(from.as_bytes()).unwrap(),
            to: Square::from_ascii(to.as_bytes()).unwrap(),
            capture: None,
            promotion: None
        };
        Some(m)
    }

    #[test]
    fn merge_single_root(){
        let t1 = TreeNode{
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 0.5,
            n: 12.0,
            q: 24.0
        };
        let roots = vec![t1.clone()];
        assert_eq!(t1, merge_trees(roots))
    }
}
