// pub fn play_2_player_game(settings: &Settings) -> Vec<Move> {
//     let mut move_history: Vec<Move> = Vec::new();
//     //TODO rename everything neposition
//     let mut game = settings.starting_position.clone();
//     let mut game_run_stats: RunStats = Default::default();
//     let mut move_num = settings.starting_move_num;
//     let mut root = TreeNode::new_root(&game, move_num);
//
//     let t0 = Instant::now();
//
//     loop {
//         let waiting_for_player = Arc::new(AtomicBool::new(true));
//         let ponder_handle = {
//             let mut ponder_game = game.clone();
//             let mut waiting_for_player_flag = waiting_for_player.clone();
//             let ponder_settings = settings.clone();
//             thread::spawn(move || {
//                 ponder(
//                     root,
//                     &ponder_game,
//                     waiting_for_player_flag,
//                     &ponder_settings,
//                 )
//             })
//         };
//         let action = stdin(&game);
//         game.play_safe(&action);
//         move_num += 0.5;
//         move_history.push(action);
//         waiting_for_player.store(false, Ordering::Relaxed);
//         let pondered_root = ponder_handle
//             .join()
//             .expect("panicked joining ponder handler");
//         println!("finding child action");
//         let after_player_move_root = find_child_action_node(pondered_root, &action)
//             .unwrap_or(TreeNode::new_root(&game, move_num)); // would only default to new if we hadn't expanded this far
//
//         let mut move_run_stats: RunStats = Default::default();
//         println!("finding best");
//         let new_root = find_best_move(after_player_move_root, &game, &mut move_run_stats, settings);
//
//         match new_root {
//             None => break,
//             Some(found_new_root) => {
//                 let best_move = found_new_root.action.unwrap();
//                 move_history.push(best_move);
//                 game.make_move(&best_move);
//                 println!("{:?}", game.board());
//                 root = found_new_root;
//             }
//         }
//         game_run_stats.add(&move_run_stats);
//
//         let pgn = pgn::to_pgn(&settings.starting_position, &move_history); //TODO build incrementally
//         println!("{}", pgn);
//
//         move_num += 0.5;
//     }
//     let time_spent = t0.elapsed().as_millis();
//     game_run_stats.total_time = time_spent as u64;
//     println!("\nGame: {}", game_run_stats);
//     move_history
// }

// }
//
//
// #[cfg(test)]
// mod tests {
//     use play::play_game;
//     use settings::*;
//
//     #[test]
//     #[ignore]
//     fn deterministic_game() {
//         let settings = Settings::lib_test_default();
//         let move_history_a = play_game(&settings);
//         let move_history_b = play_game(&settings);
//         let move_history_c = play_game(&settings);
//         assert_eq!(move_history_a, move_history_b);
//         assert_eq!(move_history_b, move_history_c);
//         assert_eq!(move_history_a, move_history_c);
//     }
//
//     #[test]
//     #[ignore]
//     fn changing_seed_changes_game() {
//         let settings_a = Settings::lib_test_default();
//         let move_history_a = play_game(&settings_a);
//         let settings_b = Settings::lib_test_default_with_seed(7);
//         let move_history_b = play_game(&settings_b);
//         assert_ne!(move_history_a, move_history_b);
//     }
// }
