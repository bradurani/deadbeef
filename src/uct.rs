use log::*;
use settings::*;
use std::cmp::Ordering::*;
use std::f32;
use tree_node::*;

// 1) exploration factor: https://www.wolframalpha.com/input/?i=chart+y%3Dsqrt(ln(1000)%2Fx)+x%3D1..1000
// 2) exploitation factor:
// 3) value factor: (none)

pub fn weight(child: &TreeNode, parent_n: u32, settings: &Settings) -> f32 {
    if !child.searchable() {
        // for sorting by weight
        return f32::MIN;
    } else if child.state == NodeState::Empty {
        // weight these above everything, sorted by board value
        // ensure they get expanded first so all roots's children get expanded,
        // and if we run out of time, the best nodes are first
        // TODO order these, mate, check etc
        //TODO change to color relative reward
        return child.color_relative_reward() as f32 + 5000.;
    }
    let weight = (child.color_relative_q() as f32 / child.n as f32)
        + settings.c * ((parent_n as f32).ln() / child.n as f32).sqrt();
    weight
}

pub fn sort_children_by_weight(children: &mut Vec<TreeNode>, parent_n: u32, settings: &Settings) {
    if cfg!(debug_assertions) {
        if children.iter().all(|c| !c.searchable()) {
            panic!("found no best children \n");
        }
    }

    children.sort_by(|a, b| {
        weight(b, parent_n, settings)
            .partial_cmp(&weight(a, parent_n, settings))
            .unwrap_or(Equal)
    });
}

pub fn most_interesting_child<'a>(
    parent: &'a mut TreeNode,
    settings: &Settings,
) -> &'a mut TreeNode {
    let parent_n = parent.n;
    parent
        .children
        .iter_mut()
        .filter(|c| c.searchable())
        .max_by(|a, b| {
            weight(a, parent_n, settings)
                .partial_cmp(&weight(b, parent_n, settings))
                .unwrap_or(Equal)
        })
        .expect("no searchable children")
}
