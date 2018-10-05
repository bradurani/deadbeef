extern crate shakmaty;

use shakmaty::{Move, Square, Role};
use mcts::TreeNode;
use std::collections::HashMap;

pub fn merge_trees(roots: Vec<TreeNode>) -> Vec<TreeNode> {
    let mut map: HashMap<Move, Vec<TreeNode>> = HashMap::new();
    for r in roots.into_iter(){
        let action_nodes = map.entry(r.action.unwrap()).or_insert(vec!());
        action_nodes.push(r);
    }
    let mut merged_roots: Vec<TreeNode> = map.into_iter().map(|(action, nodes)| merge_nodes(nodes)).collect();
    // merged_roots.sort_by(|n1, n2| n1.q.partial_cmp(&n2.q).unwrap());
    merged_roots.sort_by(|r1, r2| r1.action.partial_cmp(&r2.action).unwrap());
    merged_roots
}

pub fn merge_nodes(nodes: Vec<TreeNode>) -> TreeNode {
    let mut combined_node = nodes.first().unwrap().clone();
    combined_node.n = 0.;
    combined_node.q = 0.;
    let mut all_children = vec!();
    let mut merged_node = nodes.into_iter().fold(combined_node, |mut combined, node| {
        assert_eq!(combined.action, node.action);
        combined.n += node.n;
        combined.q += node.q;
        all_children.extend(node.children);
        combined
    });
    merged_node.children = merge_trees(all_children);
    merged_node
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
            move_num: 1.,
            n: 12.0,
            q: 24.0
        };
        let roots = vec![t1.clone()];
        assert_eq!(roots.clone(), merge_trees(roots))
    }

    #[test]
    fn merge_3_roots(){
        let t1 = TreeNode{
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            n: 12.0,
            q: 24.0
        };
        let t2 = TreeNode{
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            n: 6.0,
            q: 12.0
        };
        let t3 = TreeNode{
            action: norm('p', "d2", "d3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            n: 5.0,
            q: 10.0
        };
        let merged = TreeNode{
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            n: 18.0,
            q: 36.0
        };

        let roots = vec![t1.clone(), t2.clone(), t3.clone()];
        let expected = vec![t3.clone(), merged.clone()];
        assert_eq!(expected, merge_trees(roots))
    }

    #[test]
    fn merge_3_roots_with_3_levels_of_children(){
        let t1 = TreeNode{
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            n: 8.0,
            q: 10.0,
            children: vec![
                TreeNode{
                    action: norm('p', "f7", "f5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 7.0, q: 9.0,
                    children: vec!(
                        TreeNode{
                            action: norm('p', "d2","d3"),
                            children: vec!(),
                            state: NodeState::Expandable,
                            turn: Color::White,
                            move_num: 2.0,
                            n: 1.0, q:2.0
                        }
                    ),
                },
                TreeNode{
                    action: norm('p', "g7", "g5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 1.0, q: 1.0,
                    children: vec!(
                        TreeNode{
                            action: norm('p', "f2","f3"),
                            children: vec!(),
                            state: NodeState::Expandable,
                            turn: Color::White,
                            move_num: 2.0,
                            n: 0.0, q: 1.0
                        }
                    ),
                }
            ],
        };
        let t2 = TreeNode{
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            n: 6.0,
            q: 6.0,
            children: vec![
                TreeNode{
                    action: norm('p', "g7", "g5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 3.0, q: 3.0,
                    children: vec!(
                        TreeNode{
                            action: norm('p', "f2","f3"),
                            state: NodeState::Expandable,
                            turn: Color::White,
                            move_num: 2.0,
                            n: 0.0, q: 2.0,
                            children: vec!(),
                        }
                    ),
                },
                TreeNode{
                    action: norm('p', "h7", "h5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 3.0, q: 3.0,
                    children: vec!()
                }
            ],
        };
        let t3 = TreeNode{
            action: norm('p', "d2", "d3"),
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            n: 5.0,
            q: 10.0,
            children: vec![],
        };

        let merged = TreeNode{
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            n: 14.0,
            q: 16.0,
            children: vec![
                TreeNode{
                    action: norm('p', "f7", "f5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 7.0, q: 9.0,
                    children: vec!(
                        TreeNode{
                            action: norm('p', "d2","d3"),
                            children: vec!(),
                            state: NodeState::Expandable,
                            turn: Color::White,
                            move_num: 2.0,
                            n: 1.0, q:2.0
                        }),
                },
                TreeNode{
                    action: norm('p', "g7", "g5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 4.0, q: 4.0,
                    children: vec!(
                        TreeNode{
                            action: norm('p', "f2","f3"),
                            children: vec!(),
                            state: NodeState::Expandable,
                            turn: Color::White,
                            move_num: 2.0,
                            n: 0.0, q: 3.0
                        }
                    ),
                },
                TreeNode{
                    action: norm('p', "h7", "h5" ),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    n: 3.0, q: 3.0,
                    children: vec!(),
                }
            ],
        };
        let roots = vec![t1.clone(), t2.clone(), t3.clone()];
        let expected = vec![t3.clone(), merged.clone()];
        assert_eq!(expected, merge_trees(roots))
    }
}
