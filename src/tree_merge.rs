use mcts::NodeState;
use mcts::TreeNode;
use shakmaty::Outcome;
use stats::*;
use std::time::Instant;
use utils::deterministic_hash_map;

pub fn timed_merge_trees(
    root: TreeNode,
    new_roots: Vec<TreeNode>,
    run_stats: &mut RunStats,
) -> TreeNode {
    let t0 = Instant::now();

    let combined_root = merge_trees(root, new_roots);

    let time_spent = t0.elapsed().as_millis();
    run_stats.tree_merges += 1;
    run_stats.tree_merge_time += time_spent as u64;
    combined_root
}

fn merge_trees<'a>(mut root: TreeNode, new_roots: Vec<TreeNode>) -> TreeNode {
    debug_assert_eq!(root.nn, 0.);
    debug_assert_eq!(root.nq, 0.);
    debug_assert!(root.value.is_some());

    let mut action_map = deterministic_hash_map();

    for new_root in new_roots.into_iter() {
        debug_assert_eq!(new_root.action, root.action);
        root.sn += new_root.nn;
        root.sq += new_root.nq;
        root.outcome = max_outcome(root.outcome, new_root.outcome);
        root.state = max_state(root.state, new_root.state);
        debug_assert_eq!(root.value, new_root.value);
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
        merged_children.push(merge_trees(root_child, new_root_children));
    }
    combined_root.children = merged_children;

    combined_root
}

fn max_outcome(root_outcome: Option<Outcome>, new_outcome: Option<Outcome>) -> Option<Outcome> {
    match new_outcome {
        Some(Outcome::Decisive { winner: new_winner }) => {
            if cfg!(debug_assertions) {
                match root_outcome {
                    Some(Outcome::Decisive { winner }) => assert_eq!(new_winner, winner),
                    _ => {}
                }
            }
            new_outcome
        }
        Some(Outcome::Draw) => {
            if cfg!(debug_assertions) {
                match root_outcome {
                    Some(Outcome::Decisive { winner: _winner }) => {
                        panic!("new outcome draw but root was decisive")
                    }
                    _ => {}
                }
            }
            new_outcome
        }
        None => root_outcome,
    }
}

fn max_state(root_outcome: NodeState, new_outcome: NodeState) -> NodeState {
    match new_outcome {
        NodeState::LeafNode => NodeState::LeafNode,
        NodeState::FullyExpanded => match root_outcome {
            NodeState::LeafNode => NodeState::LeafNode,
            _ => NodeState::FullyExpanded,
        },
        NodeState::Expandable => root_outcome,
    }
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
        let root = TreeNode::new(norm('p', "e2", "e3"), Color::White, 1., Some(0));

        let t1 = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            value: Some(0),
            nn: 12.0,
            nq: 24.0,
            sn: 0.,
            sq: 0.,
        };

        let expected = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            value: Some(0),
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
            outcome: None,
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            value: Some(0),
            nn: 0.,
            nq: 0.,
            sn: 500.0,
            sq: 100.0,
        };
        let t1 = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            value: Some(0),
            nn: 12.0,
            nq: 24.0,
            sn: 1000.,
            sq: 2000.,
        };
        let t2 = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            value: Some(0),
            nn: 6.0,
            nq: 12.0,
            sn: 0.,
            sq: 0.,
        };
        let expected = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            children: vec![],
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 1.,
            value: Some(0),
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
            outcome: None,
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            value: Some(2),
            children: vec![],
            nn: 0.,
            nq: 0.,
            sn: 500.,
            sq: 100.,
        };
        let t1 = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            value: Some(2),
            nn: 8.0,
            nq: 10.0,
            sn: 0.,
            sq: 0.,
            children: vec![
                TreeNode {
                    outcome: None,
                    action: norm('p', "f7", "f5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    value: Some(12),
                    nn: 7.0,
                    nq: 9.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![TreeNode {
                        outcome: None,
                        action: norm('p', "d2", "d3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        value: Some(10),
                        nn: 1.0,
                        nq: 2.0,
                        sn: 0.,
                        sq: 0.,
                    }],
                },
                TreeNode {
                    outcome: None,
                    action: norm('p', "g7", "g5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    value: Some(14),
                    nn: 1.0,
                    nq: 1.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![TreeNode {
                        outcome: None,
                        action: norm('p', "f2", "f3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        value: Some(9),
                        nn: 0.0,
                        nq: 1.0,
                        sn: 0.,
                        sq: 0.,
                    }],
                },
            ],
        };
        let t2 = TreeNode {
            outcome: None,
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            value: Some(2),
            nn: 6.0,
            nq: 6.0,
            sn: 0.,
            sq: 0.,
            children: vec![
                TreeNode {
                    outcome: None,
                    action: norm('p', "g7", "g5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    value: Some(14),
                    nn: 3.0,
                    nq: 3.0,
                    sn: 0.,
                    sq: 0.,
                    children: vec![TreeNode {
                        outcome: None,
                        action: norm('p', "f2", "f3"),
                        state: NodeState::Expandable,
                        turn: Color::White,
                        value: Some(9),
                        move_num: 2.0,
                        nn: 0.0,
                        nq: 2.0,
                        sn: 0.,
                        sq: 0.,
                        children: vec![],
                    }],
                },
                TreeNode {
                    outcome: None,
                    action: norm('p', "h7", "h5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    value: Some(100),
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
            outcome: None,
            action: norm('p', "e2", "e3"),
            state: NodeState::FullyExpanded,
            turn: Color::White,
            move_num: 1.,
            value: Some(2),
            nn: 0.,
            nq: 0.,
            sn: 514.0,
            sq: 116.0,
            children: vec![
                TreeNode {
                    outcome: None,
                    action: norm('p', "g7", "g5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    value: Some(14),
                    nn: 0.,
                    nq: 0.,
                    sn: 4.0,
                    sq: 4.0,
                    children: vec![TreeNode {
                        outcome: None,
                        action: norm('p', "f2", "f3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        value: Some(9),
                        nn: 0.,
                        nq: 0.,
                        sn: 0.0,
                        sq: 3.0,
                    }],
                },
                TreeNode {
                    outcome: None,
                    action: norm('p', "h7", "h5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    value: Some(100),
                    nn: 0.,
                    nq: 0.,
                    sn: 3.0,
                    sq: 3.0,
                    children: vec![],
                },
                TreeNode {
                    outcome: None,
                    action: norm('p', "f7", "f5"),
                    state: NodeState::Expandable,
                    turn: Color::Black,
                    move_num: 1.5,
                    value: Some(12),
                    nn: 0.,
                    nq: 0.,
                    sn: 7.0,
                    sq: 9.0,
                    children: vec![TreeNode {
                        outcome: None,
                        action: norm('p', "d2", "d3"),
                        children: vec![],
                        state: NodeState::Expandable,
                        turn: Color::White,
                        move_num: 2.0,
                        value: Some(10),
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
