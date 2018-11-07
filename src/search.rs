// #[cfg(test)]
// mod tests {
//     use search::*;
//     use shakmaty::fen::Fen;
//     use stats::{RunStats, TreeStats};
//
//     #[test]
//     #[ignore]
//     fn search_deterministic_starting_pos() {
//         fn run_search() -> TreeNode {
//             let settings = Settings::lib_test_default();
//             let mut test_run_stats: RunStats = Default::default();
//             let game = &Chess::default();
//             let root = TreeNode::new_root(game, 0.5);
//             search(root, game, &mut test_run_stats, &settings)
//         }
//         let a = run_search();
//         let b = run_search();
//         let c = run_search();
//         println!(
//             "{:?}\n{:?}\n{:?}",
//             TreeStats::tree_stats(&a),
//             TreeStats::tree_stats(&b),
//             TreeStats::tree_stats(&c)
//         );
//         assert_eq!(a, b);
//         assert_eq!(b, c);
//         assert_eq!(a, c);
//     }
//
//     #[test]
//     #[ignore]
//     fn run_search_deterministic_middle_game_position() {
//         fn run_search() -> TreeNode {
//             let setup: Fen = "rn3rk1/pbppq1pp/1p2pb2/4N2Q/3PN3/3B4/PPP2PPP/R3K2R w KQ - 6 11"
//                 .parse()
//                 .unwrap();
//             let game: Chess = setup.position().unwrap();
//             let settings = Settings::lib_test_default();
//             let root = TreeNode::new_root(&game, 1.);
//             let mut test_run_stats: RunStats = Default::default();
//             search(root, &game, &mut test_run_stats, &settings)
//         }
//         let a = run_search();
//         let b = run_search();
//         let c = run_search();
//         println!(
//             "{:?}\n{:?}\n{:?}",
//             TreeStats::tree_stats(&a),
//             TreeStats::tree_stats(&b),
//             TreeStats::tree_stats(&c)
//         );
//         assert_eq!(a, b);
//         assert_eq!(b, c);
//         assert_eq!(a, c);
//     }
// }
