use game::*;
use mcts::TreeNode;
use pgn;
use search::*;
use settings::*;
use shakmaty::*;
use stats::*;
use std::time::Instant;

pub fn play_game(settings: &Settings) -> Vec<Move> {
    let mut move_history: Vec<Move> = Vec::new();
    //TODO rename everything position
    let mut game = settings.starting_position.clone();
    let mut game_run_stats: RunStats = Default::default();
    let mut move_num = settings.starting_move_num;
    let mut root = TreeNode::new_root(&game, move_num);

    let t0 = Instant::now();

    loop {
        let mut move_run_stats: RunStats = Default::default();
        let new_root = find_best_move(root, &game, &mut move_run_stats, settings);

        match new_root {
            None => break,
            Some(found_new_root) => {
                let best_move = found_new_root.action.unwrap();
                move_history.push(best_move);
                game.make_move(&best_move);
                root = found_new_root;

                println!("{:?}", game.board());
                println!("Move: {}", best_move);
            }
        }
        game_run_stats.add(&move_run_stats);

        let pgn = pgn::to_pgn(&settings.starting_position, &move_history); //TODO build incrementally
        println!("{}", pgn);

        move_num += 0.5;
    }
    let time_spent = t0.elapsed().as_millis();
    game_run_stats.total_time = time_spent as u64;
    println!("\nGame: {}", game_run_stats);
    move_history
}

pub fn find_best_move(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> Option<TreeNode> {
    let t0 = Instant::now();

    println!(
        "-------------------------------------\n{}    {} / {}  s: {}",
        root.move_num,
        root.sq,
        root.sn,
        root.score()
    );

    if root.is_game_over_or_drawn(game) {
        return None;
    }

    let new_root = search(root, &game, move_run_stats, settings);

    let best_child = best_child_node(new_root);

    let time_spent = t0.elapsed().as_millis();
    move_run_stats.total_time = time_spent as u64;
    println!("{}", move_run_stats);

    Some(best_child)
}

fn best_child_node(root: TreeNode) -> TreeNode {
    debug_assert_eq!(0., root.nn); // shoud have a merged node with no new calculations
    debug_assert_eq!(0., root.nq);
    // TODO try the equation from the MCTS-Solver paper
    root.children
        .into_iter()
        .max_by(|n1, n2| {
            n1.color_relative_score()
                .partial_cmp(&n2.color_relative_score())
                .unwrap()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use play::play_game;
    use settings::*;

    #[test]
    #[ignore]
    fn deterministic_game() {
        let settings = Settings::lib_test_default();
        let move_history_a = play_game(&settings);
        let move_history_b = play_game(&settings);
        let move_history_c = play_game(&settings);
        assert_eq!(move_history_a, move_history_b);
        assert_eq!(move_history_b, move_history_c);
        assert_eq!(move_history_a, move_history_c);
    }

    #[test]
    #[ignore]
    fn changing_seed_changes_game() {
        let settings_a = Settings::lib_test_default();
        let move_history_a = play_game(&settings_a);
        let settings_b = Settings::lib_test_default_with_seed(7);
        let move_history_b = play_game(&settings_b);
        assert_ne!(move_history_a, move_history_b);
    }
}
