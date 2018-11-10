use log::*;
use mcts::*;
use settings::*;
use std::cmp::Ordering::*;
use std::f32;

// 1) exploration factor: https://www.wolframalpha.com/input/?i=chart+y%3Dsqrt(ln(1000)%2Fx)+x%3D1..1000
// 2) exploitation factor:
// 3) value factor: (none)

pub fn weight(child: &TreeNode, parent_n: f32, settings: &Settings) -> f32 {
    if child.outcome.is_some() {
        // for sorting by weight. Tiny optimization, can skip this check during traversal?
        return 0.;
    };
    let weight = (child.adjusted_q() / child.n) + settings.c * (parent_n.ln() / child.n).sqrt();
    weight
}

pub fn sort_children_by_weight(children: &mut Vec<TreeNode>, parent_n: f32, settings: &Settings) {
    if cfg!(debug_assertions) {
        if !children.iter().any(|c| c.state != NodeState::LeafNode) {
            error!("found no best children \n");
        }
    }

    children.sort_by(|a, b| {
        if a.state == NodeState::LeafNode && b.state == NodeState::LeafNode {
            Equal
        } else if a.state == NodeState::LeafNode {
            Greater
        } else if b.state == NodeState::LeafNode {
            Less
        } else {
            weight(b, parent_n, settings)
                .partial_cmp(&weight(a, parent_n, settings))
                .unwrap_or(Equal)
        }
    });
}

pub fn best_child<'a>(parent: &'a mut TreeNode, settings: &Settings) -> &'a mut TreeNode {
    if cfg!(debug_assertions) {
        if !parent
            .children
            .iter()
            .any(|c| c.state != NodeState::LeafNode)
        {
            panic!("no children in best_child");
        }
    }

    let parent_n = parent.n;
    parent
        .children
        .iter_mut()
        .filter(|c| c.state != NodeState::LeafNode)
        .max_by(|a, b| {
            weight(a, parent_n, settings)
                .partial_cmp(&weight(b, parent_n, settings))
                .unwrap_or(Equal)
        })
        .unwrap()
}
