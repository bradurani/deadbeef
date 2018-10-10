use game::*;
use mcts::{TreeNode, MCTS};
use pgn;
use shakmaty::{Chess, Move};
use std::time::Instant;

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
    let mut root = TreeNode::new_root(&game, move_num);

    loop {
        move_num += 0.5;
        let new_root = find_best_move(
            &mut mcts,
            root,
            &game,
            ensemble_size,
            time_per_move_ms,
            c,
            move_num,
            n_samples,
        );
        match new_root {
            None => break,
            Some(found_new_root) => {
                let best_move = found_new_root.action.unwrap();
                move_history.push(best_move);
                game.make_move(&best_move);
                root = found_new_root.clone();
            }
        }
        let pgn = pgn::to_pgn(&starting_position, &move_history); //TODO build incrementally
        println!("{}", pgn);
    }
    move_history
}

pub fn find_best_move(
    mcts: &mut MCTS,
    root: TreeNode,
    game: &Chess,
    ensemble_size: usize,
    time_per_move_ms: f32,
    c: f32,
    move_num: f32,
    n_samples: isize,
) -> Option<TreeNode> {
    println!("\nMove: {}", move_num);
    let t0 = Instant::now();

    // println!("Starting with {:?}", mcts.tree_statistics(&vec![root]));

    let new_root = if n_samples == -1 {
        mcts.search_time(root, &game, ensemble_size, time_per_move_ms, c)
    } else {
        mcts.search(root, &game, ensemble_size, n_samples as usize, c)
    };

    // println!("{}", new_root);
    // println!("Calculated {:?}", mcts.tree_statistics(root));

    let best_child = best_child_node(new_root);

    let time_spend = t0.elapsed().as_millis();
    println!("move time: {}ms", time_spend);
    best_child
}

fn best_child_node(root: TreeNode) -> Option<TreeNode> {
    assert_eq!(0., root.nn); // shoud have a merged node with no new calculations
    assert_eq!(0., root.nq);
    root.children
        .into_iter()
        .max_by(|n1, n2| n1.sn.partial_cmp(&n2.sn).unwrap())
}
