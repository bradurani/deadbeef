use mcts::TreeNode;
use utils::deterministic_hash_map;

pub fn merge_trees<'a>(mut root: TreeNode, new_roots: Vec<TreeNode>) -> TreeNode {
    assert_eq!(root.nn, 0.);
    assert_eq!(root.nq, 0.);

    let mut action_map = deterministic_hash_map();

    for new_root in new_roots.into_iter() {
        assert_eq!(new_root.action, root.action);
        root.sn += new_root.nn;
        root.sq += new_root.nq;
        for new_child_root in new_root.children {
            let grouped_new_child_roots = action_map
                .entry(new_child_root.action.unwrap())
                .or_insert(vec![]);
            grouped_new_child_roots.push(new_child_root);
        }
    }
    // println!("action_map\n{:#?}", action_map);

    let mut combined_root = root.clone_childless();

    let mut root_action_map = deterministic_hash_map();
    for root_child in root.children {
        root_action_map.insert(root_child.action.unwrap(), root_child);
    }

    let mut merged_children = vec![];
    for (action, new_root_children) in action_map.into_iter() {
        let root_child = match root_action_map.remove(&action) {
            Some(found_child) => found_child,
            None => new_root_children.first().unwrap().clone_empty(),
        };
        println!("recursing");
        merged_children.push(merge_trees(root_child, new_root_children));
    }
    combined_root.children = merged_children;
    combined_root
}

#[cfg(test)]
mod tests {

    use mcts::{NodeState, TreeNode};
    use shakmaty::{Color, Move, Role, Square};
    use tree_merge::merge_trees;

    fn norm(role: char, from: &'static str, to: &'static str) -> Option<Move> {
        let m = Move::Normal {
            role: Role::from_char(role).unwrap(),
            from: Square::from_ascii(from.as_bytes()).unwrap(),
            to: Square::from_ascii(to.as_bytes()).unwrap(),
            capture: None,
            promotion: None,
        };
        Some(m)
    }

    #[test]
    fn merge_single_root() {
        let root = TreeNode::new(norm('p', "e2", "e3"), Color::White, 1.);

        let t1 = TreeNode {
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            nn: 12.0,
            nq: 24.0,
            sn: 0.,
            sq: 0.,
        };

        let expected = TreeNode {
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            nn: 0.0,
            nq: 0.0,
            sn: 12.,
            sq: 24.,
        };

        let new_root = merge_trees(root, vec![t1]);
        assert_eq!(expected, new_root);
    }

    #[test]
    fn merge_3_roots() {
        let root = TreeNode {
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            nn: 0.,
            nq: 0.,
            sn: 500.0,
            sq: 100.0,
        };
        let t1 = TreeNode {
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            nn: 12.0,
            nq: 24.0,
            sn: 1000.,
            sq: 2000.,
        };
        let t2 = TreeNode {
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            nn: 6.0,
            nq: 12.0,
            sn: 0.,
            sq: 0.,
        };
        let expected = TreeNode {
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            nn: 0.,
            nq: 0.,
            sn: 518.,
            sq: 136.,
        };

        let roots = vec![t1, t2];
        let new_root = merge_trees(root, roots);
        assert_eq!(expected, new_root);
    }

    #[test]
    fn merge_3_roots_with_3_levels_of_children() {
        let root = TreeNode {
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            children: vec![],
            nn: 0.,
            nq: 0.,
            sn: 500.,
            sq: 100.,
        };
        let t1 = TreeNode {
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            nn: 8.0,
            nq: 10.0,
            sn: 0.,
            sq: 0.,
            children: vec![
                TreeNode {
                    action: norm('p', "f7", "f5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 7.0,
                    nq: 9.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![TreeNode {
                        action: norm('p', "d2", "d3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        nn: 1.0,
                        nq: 2.0,
                        sn: 0.,
                        sq: 0.,
                    }],
                },
                TreeNode {
                    action: norm('p', "g7", "g5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 1.0,
                    nq: 1.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![TreeNode {
                        action: norm('p', "f2", "f3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        nn: 0.0,
                        nq: 1.0,
                        sn: 0.,
                        sq: 0.,
                    }],
                },
            ],
        };
        let t2 = TreeNode {
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            nn: 6.0,
            nq: 6.0,
            sn: 0.,
            sq: 0.,
            children: vec![
                TreeNode {
                    action: norm('p', "g7", "g5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 3.0,
                    nq: 3.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![TreeNode {
                        action: norm('p', "f2", "f3"),
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        nn: 0.0,
                        nq: 2.0,
                        sn: 0.,
                        sq: 0.,
                        children: vec![],
                    }],
                },
                TreeNode {
                    action: norm('p', "h7", "h5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 3.0,
                    nq: 3.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![],
                },
            ],
        };
        let expected = TreeNode {
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            nn: 0.,
            nq: 0.,
            sn: 514.0,
            sq: 116.0,
            children: vec![
                TreeNode {
                    action: norm('p', "g7", "g5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 0.,
                    nq: 0.,
                    sn: 4.0,
                    sq: 4.0,
                    children: vec![TreeNode {
                        action: norm('p', "f2", "f3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        nn: 0.,
                        nq: 0.,
                        sn: 0.0,
                        sq: 3.0,
                    }],
                },
                TreeNode {
                    action: norm('p', "h7", "h5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 0.,
                    nq: 0.,
                    sn: 3.0,
                    sq: 3.0,
                    children: vec![],
                },
                TreeNode {
                    action: norm('p', "f7", "f5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    nn: 0.,
                    nq: 0.,
                    sn: 7.0,
                    sq: 9.0,
                    children: vec![TreeNode {
                        action: norm('p', "d2", "d3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        nn: 0.,
                        nq: 0.,
                        sn: 1.0,
                        sq: 2.0,
                    }],
                },
            ],
        };
        let roots = vec![t1, t2];
        let new_root = merge_trees(root, roots);
        assert_eq!(expected, new_root);
    }
}
