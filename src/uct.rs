use game::*;
use mcts::*;
use settings::*;
use std::f32;

pub fn best_child<'a>(parent: &'a mut TreeNode, settings: &Settings) -> &'a mut TreeNode {
    // println!("\n--");
    // println!("best_child for: {}", parent);
    if cfg!(debug_assertions) {
        if !parent
            .children
            .iter()
            .any(|c| c.state != NodeState::LeafNode)
        {
            println!("found no best children \n{}", parent);
        }
    }

    let mut best_weight: f32 = f32::NEG_INFINITY;
    let mut best_child: Option<&mut TreeNode> = None;
    let parent_total_n = parent.total_n();
    //TODO try alpha zerp version, MCTS-Solver version and Wikipedia weighted version (are they
    //the same) can eval be used as any of the factors
    for child in &mut parent.children {
        // println!("child: {}", child);
        if child.state == NodeState::LeafNode {
            continue;
        }
        let weight = weight(child, parent.turn.coefficient(), parent_total_n, settings);
        // println!("values weight {}", weight);
        // weight = weight.max(-1.).min(1.);
        // println!("weighted weight {}", weight);
        // println!("value {}", value);
        //TODO what is this 2. ?????
        // println!("child: {:?} total: {}", child, child.total_n());
        // println!("value: {}", value);
        if weight > best_weight {
            best_weight = weight;
            best_child = Some(child);
        }
    }
    let found_best_child = best_child.unwrap();
    found_best_child
}

fn weight(
    child: &TreeNode,
    parent_coefficient: f32,
    parent_total_n: f32,
    settings: &Settings,
) -> f32 {
    let mut weight = (parent_coefficient * child.total_q()) / child.total_n()
        + settings.c * (parent_total_n.ln() / child.total_n()).sqrt();
    // println!("raw weight {}", weight);
    weight += child.normalized_color_relative_value(); // / child.total_n();
    weight
}
