use std::sync::Once;


// #![allow(dead_code)]
//
// extern crate deadbeef;
// extern crate shakmaty;
//
// use deadbeef::display::*;
// use deadbeef::mcts::TreeNode;
// use deadbeef::play;
// use deadbeef::settings::*;
// use deadbeef::setup::*;
// use deadbeef::stats::*;
// use shakmaty::*;
//
// pub fn assert_move(fen_str: &'static str, uci_str: &'static str) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_default();
//     assert_contains_move_with_settings(
//         fen_str,
//         vec![uci_str],
//         &mut test_run_stats,
//         &settings,
//         false,
//     );
// }
//
// pub fn assert_mate_move(fen_str: &'static str, uci_str: &'static str) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_mate_default();
//     assert_contains_move_with_settings(
//         fen_str,
//         vec![uci_str],
//         &mut test_run_stats,
//         &settings,
//         true,
//     );
// }
//
// pub fn assert_contains_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_default();
//     assert_contains_move_with_settings(fen_str, uci_strs, &mut test_run_stats, &settings, false)
// }
//
// pub fn assert_contains_mate_move(fen_str: &'static str, uci_strs: Vec<&'static str>) {
//     let mut test_run_stats: RunStats = Default::default();
//     let settings = Settings::test_mate_default();
//     assert_contains_move_with_settings(fen_str, uci_strs, &mut test_run_stats, &settings, true)
// }
//
// pub fn assert_draw(fen_str: &'static str, stats: &mut RunStats) {
//     let settings = Settings::test_mate_default();
//     let position = parse_fen(fen_str);
//     println!("halfmove clock: {}", position.halfmoves());
//     let root = TreeNode::new_root(&position, 100.);
//
//     println!("{}", settings);
//
//     let new_root = play::find_best_move(root, &position, stats, &settings);
//
//     assert!(new_root.map_or(false, |o| o.is_draw()));
// }
//
// pub fn assert_contains_move_with_settings(
//     fen_str: &'static str,
//     uci_strs: Vec<&'static str>,
//     test_run_stats: &mut RunStats,
//     settings: &Settings,
//     assert_mate: bool,
// ) {
//     println!("\nevaluating: {}", fen_str);
//     let position = parse_fen(fen_str);
//     let moves: Vec<Move> = uci_strs.iter().map(|u| parse_uci(u, &position)).collect();
//
//     println!("{}", settings);
//
//     let best_child = play::find_best_move(
//         TreeNode::new_root(&position, settings.starting_iterations_per_ms),
//         &position,
//         test_run_stats,
//         &settings,
//     )
//     .unwrap();
//     print_tree(&best_child, &settings);
//     let best_action = best_child.action.unwrap();
//     if assert_mate {
//         assert!(best_child.is_decisive());
//     }
//     if moves.contains(&best_action) {
//         assert!(true);
//     } else {
//         let moves_str: Vec<String> = moves.iter().map(|m| format!("{}", m)).collect();
//         panic!("{} not found in {}", best_action, moves_str.join(" ,"));
//     }
// }
