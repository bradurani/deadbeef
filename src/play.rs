extern crate matches;
extern crate shakmaty;

use mcts::{Game, TreeNode, MCTS};
use pgn;
use shakmaty::{Chess, Move, Setup};
use std::time::Instant;
use tree_merge::merge_trees;

pub fn play_game(
    starting_position: &Chess,
    ensemble_size: usize,
    time_per_move_ms: f32,
    c: f32,
    starting_seed: u8,
    n_samples: isize,
) -> Vec<Move> {
    let mut game = starting_position.clone();
    let mut move_history: Vec<Move> = Vec::new();
    let mut move_num = 0.5;
    let mut mcts: MCTS = MCTS::new(starting_seed);
    let mut merged_root = TreeNode::new_root(&game, move_num);

    loop {
        move_num += 0.5;
        let action = make_move(
            &mut mcts,
            merged_root,
            &game,
            ensemble_size,
            time_per_move_ms,
            c,
            move_num,
            n_samples,
        );
        match action {
            None => break,
            Some((action, new_root)) => {
                move_history.push(action);
                game.make_move(&action);
                merged_root = new_root;
            }
        }
        let pgn = pgn::to_pgn(&starting_position, &move_history); //TODO build incrementally
        println!("{}", pgn);
    }
    move_history
}

pub fn make_move(
    mcts: &mut MCTS,
    root: TreeNode,
    game: &Chess,
    ensemble_size: usize,
    time_per_move_ms: f32,
    c: f32,
    move_num: f32,
    n_samples: isize,
) -> Option<(Move, TreeNode)> {
    println!("\nMove: {}", move_num);
    let t0 = Instant::now();

    println!(
        "Starting with {:?}",
        mcts.tree_statistics(&vec![root.clone()])
    );

    let roots = if n_samples == -1 {
        mcts.search_time(&root, &game, ensemble_size, time_per_move_ms, c)
    } else {
        mcts.search(&root, &game, ensemble_size, n_samples as usize, c)
    };

    println!("Calculated {:?}", mcts.tree_statistics(&roots));
    // DEBUG
    {
        let debug_combined_node = merge_trees(roots.clone());
        println!("{}", debug_combined_node);
    }
    // DEBUG

    let best_children = mcts.best_children(roots);
    let action_and_new_root = best_children.map(|children| {
        let new_root = merge_trees(children);
        let action = new_root.action.unwrap();
        println!("Moving: {}\n{:?}", &action, game.board());
        (action, new_root)
    });
    let time_spend = t0.elapsed().as_millis();
    println!("move time: {}ms", time_spend);
    action_and_new_root
}
