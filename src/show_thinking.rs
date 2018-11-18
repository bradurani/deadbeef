use game::*;
use mcts::*;
use settings::*;
use shakmaty::san::*;
use shakmaty::Chess;
use stats::*;
use std::fmt;

pub fn show_thinking(root: &TreeNode, position: &mut Chess, stats: &RunStats, settings: &Settings) {
    let elapsed_cs = stats.elapsed().as_millis() / 10;
    let best_path = iterate_best_path(root, position);
    let depth = best_path.path.len();
    let selective_depth = depth;
    let speed = stats.evals_per_second();
    let tablebase_hits = 0;
    if settings.show_thinking && stats.batches % settings.show_thinking_freq == 0 {
        println!(
            "{} {} {} {} {} {} {} \t{}",
            depth,
            root.minimax,
            elapsed_cs,
            stats.evals,
            selective_depth,
            speed,
            tablebase_hits,
            best_path
        );
    }
}

#[derive(Default, Debug)]
struct BestPath {
    path: Vec<SanPlus>,
}

fn iterate_best_path(root: &TreeNode, position: &mut Chess) -> BestPath {
    let mut best_path: BestPath = Default::default();
    let mut head = root;
    while !head.children.is_empty() {
        head = head
            .children
            .iter()
            .max_by(|n1, n2| {
                n1.color_relative_score()
                    .partial_cmp(&n2.color_relative_score())
                    .unwrap()
            })
            .unwrap();
        let action = head.action.clone().unwrap();
        best_path
            .path
            .push(SanPlus::from_move(position.clone(), &action));
        position.make_move(&action);
    }
    best_path
}

impl fmt::Display for BestPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for action in self.path.iter() {
            f.write_fmt(format_args!("{} ", action))?;
        }
        Ok(())
    }
}