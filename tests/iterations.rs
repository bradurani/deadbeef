// extern crate deadbeef;
// extern crate shakmaty;
//
// use helpers::*;
// use std::sync::Once;
//
// mod helpers;
//
// use deadbeef::display::*;
// use deadbeef::emojify::*;
// use deadbeef::logger;
// use deadbeef::mcts::*;
// use deadbeef::search_strategy::*;
// use deadbeef::settings::*;
// use deadbeef::setup::*;
// use deadbeef::stats::*;
// use deadbeef::utils::*;
// use shakmaty::Color::*;
// use shakmaty::*;
//
// #[test]
// fn test_iteration_mate_in_1() {
//     let mut stats: RunStats = Default::default();
//     let (node, delta) = test_iteration_until_outcome("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", &mut stats);
//     assert_eq!(Some(White), node.winner());
//     assert_eq!(White, node.turn);
//     assert!(stats.iterations < 50);
//     assert_eq!(NodeState::LeafNode, node.state);
//     assert_eq!(delta, 1.);
// }
//
// #[test]
// fn test_decisive_if_child_is_win() {
//     let mut stats: RunStats = Default::default();
//     let (node, delta) = test_iteration_until_outcome("8/8/8/8/8/p2k4/r7/3K4 b - - 0 1", &mut stats);
//     assert_eq!(Some(Black), node.winner());
//     assert_eq!(Black, node.turn);
//     assert!(stats.iterations < 20);
//     assert_eq!(NodeState::LeafNode, node.state);
//     assert_eq!(-1., delta);
// }
//
// #[test]
// fn test_delta_1_in_dominate_position() {
//     let mut stats: RunStats = Default::default();
//     let (node, delta) =
//         test_single_iteration("2r1q2k/4p2n/4P3/8/3NK3/1p6/2p5/8 w - - 0 1", &mut stats);
//     assert_eq!(None, node.outcome);
//     assert_eq!(Color::White, node.turn);
//     assert_eq!(NodeState::Expandable, node.state);
//     assert!(delta < -1.);
// }
//
// // #[test]
// // fn test_sets_min_score_if_child_is_draw() {
// //     let mut stats: RunStats = Default::default();
// //     let (node, delta) = test_iteration_to_fully_expanded(
// //         "8/2kr4/8/8/8/3pK3/3Q4/8 b - - 0 1",
// //         vec![
// //             "8/2k1r3/8/8/8/3pK3/3Q4/8 w - - 0 1",
// //             "8/2k1r3/8/8/8/3pK3/3Q4/8 w - - 0 1",
// //         ],
// //         &mut stats,
// //     );
// //     assert_eq!(0., delta); //black is behind but has an option to draw, so delta is 0
// //     assert_eq!(1., node.n);
// //     assert_eq!(None, node.outcome);
// //     assert_eq!(Color::Black, node.turn);
// //     assert_eq!(NodeState::Expandable, node.state);
// //     assert_eq!(node.min_score, Some(0));
// //     assert_eq!(node.max_score, None);
// // }
// //
// // #[test]
// // fn test_is_draw_if_all_children_are_draws() {
// //     let mut stats: RunStats = Default::default();
// //     let game = parse_fen("q4rk1/5p2/8/6Q1/8/8/8/6K1 b - - 3 2");
// //     let mut repetition_detector = RepetitionDetector::new(&game);
// //     let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
// //     let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
// //     repetition_detector.record_and_check(&drawing_position_1);
// //     repetition_detector.record_and_check(&drawing_position_1);
// //     repetition_detector.record_and_check(&drawing_position_2);
// //     repetition_detector.record_and_check(&drawing_position_2);
// //     let mut node = TreeNode {
// //         outcome: None,
// //         action: None,
// //         children: vec![],
// //         state: NodeState::Expandable,
// //         turn: Color::Black,
// //         move_num: 12.,
// //         value: Some(-100), //TODO make value not an option
// //         repetition_detector: repetition_detector,
// //         max_score: None,
// //         min_score: None,
// //         n: 1.,
// //         q: 0.,
// //     };
// //     let settings = Settings::lib_test_default();
// //     let seed = 6;
// //     node.iteration(
// //         &mut game.clone(),
// //         &mut seeded_rng(seed),
// //         &mut stats,
// //         &settings,
// //     );
// //     let delta = node.iteration(
// //         &mut game.clone(),
// //         &mut seeded_rng(seed),
// //         &mut stats,
// //         &settings,
// //     );
// //     print_tree(&node, &settings);
// //     assert_eq!(0., delta);
// //     assert_eq!(3., node.n);
// //     assert_eq!(Some(Outcome::Draw), node.outcome);
// //     assert_eq!(Color::Black, node.turn);
// //     assert_eq!(NodeState::LeafNode, node.state);
// //     assert_eq!(Some(0), node.max_score);
// //     assert_eq!(Some(0), node.min_score);
// // }
// //
// // #[test]
// // fn test_sets_min_score_if_opponent_can_force_draw() {
// //     let mut stats: RunStats = Default::default();
// //     let game = parse_fen("q4rk1/5p2/8/6Q1/8/8/8/6K1 b - - 3 2");
// //     let mut repetition_detector = RepetitionDetector::new(&game);
// //     let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
// //     let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
// //     repetition_detector.record_and_check(&drawing_position_1);
// //     repetition_detector.record_and_check(&drawing_position_1);
// //     repetition_detector.record_and_check(&drawing_position_2);
// //     let mut node = TreeNode {
// //         outcome: None,
// //         action: None,
// //         children: vec![],
// //         state: NodeState::Expandable,
// //         turn: Color::Black,
// //         move_num: 12.,
// //         value: Some(-100), //TODO make value not an option
// //         repetition_detector: repetition_detector,
// //         max_score: None,
// //         min_score: None,
// //         n: 1.,
// //         q: 0.,
// //     };
// //     let settings = Settings::lib_test_default();
// //     let seed = 6;
// //     let mut delta = 0.;
// //     let n = 20000.;
// //     for _i in 0..n as u32 {
// //         delta = node.iteration(
// //             &mut game.clone(),
// //             &mut seeded_rng(seed),
// //             &mut stats,
// //             &settings,
// //         );
// //         if node.max_score.is_some() || node.outcome.is_some() {
// //             break;
// //         }
// //     }
// //     print_tree(&node, &settings);
// //     assert_eq!(None, node.outcome);
// //     // assert_eq!(-1., delta);
// //     assert_eq!(n + 1., node.n);
// //     assert_eq!(Color::Black, node.turn);
// //     assert_eq!(NodeState::FullyExpanded, node.state);
// //     assert_eq!(None, node.max_score);
// //     assert_eq!(Some(0), node.min_score);
// // }
// //
// // #[test]
// // fn test_outcome_is_draw_if_lose_or_draw() {
// //     let mut stats: RunStats = Default::default();
// //     let game = parse_fen("1q3k2/8/8/8/8/8/r7/6K1 w - - 1 1");
// //     let mut repetition_detector = RepetitionDetector::new(&game);
// //     let drawing_position = parse_fen("1q3k2/8/8/8/8/8/r7/5K2 b - - 2 1");
// //     repetition_detector.record_and_check(&drawing_position);
// //     repetition_detector.record_and_check(&drawing_position);
// //     let mut node = TreeNode {
// //         outcome: None,
// //         action: None,
// //         children: vec![],
// //         state: NodeState::Expandable,
// //         turn: Color::White,
// //         move_num: 12.,
// //         value: Some(-100), //TODO make value not an option
// //         repetition_detector: repetition_detector,
// //         max_score: None,
// //         min_score: None,
// //         n: 1.,
// //         q: 0.,
// //     };
// //     let settings = Settings::lib_test_default();
// //     let seed = 1;
// //     let mut delta = 0.;
// //     let n = 17.;
// //     for _i in 0..n as u32 {
// //         delta = node.iteration(
// //             &mut game.clone(),
// //             &mut seeded_rng(seed),
// //             &mut stats,
// //             &settings,
// //         );
// //         if node.outcome.is_some() {
// //             break;
// //         }
// //     }
// //     print_tree(&node, &settings);
// //     assert_eq!(Some(Outcome::Draw), node.outcome);
// //     // assert_eq!(-1., delta);
// //     assert_eq!(n + 1., node.n);
// //     assert_eq!(Color::White, node.turn);
// //     assert_eq!(NodeState::LeafNode, node.state);
// //     assert_eq!(Some(0), node.min_score);
// //     assert_eq!(Some(0), node.max_score);
// // }
// //
// // #[test]
// // fn test_draws_intentionally_if_behind() {
// //     // test that if there's a draw on the next move and we're way behind, we draw,
// //     // even though exploration will rack up n for other nodes, so we can't just
// //     // rely on n to choose best move
// //     let mut stats: RunStats = Default::default();
// //     let game = parse_fen("n1nqk3/b1ppp3/1p6/p7/5P2/4P1P1/4NK2/8 w - - 0 1");
// //     let mut repetition_detector = RepetitionDetector::new(&game);
// //     let drawing_position = parse_fen("n1nqk3/b1ppp3/1p6/p7/5P2/4P1P1/4N3/5K2 b - - 0 1");
// //     repetition_detector.record_and_check(&drawing_position);
// //     repetition_detector.record_and_check(&drawing_position);
// //     let mut node = TreeNode {
// //         outcome: None,
// //         action: None,
// //         children: vec![],
// //         state: NodeState::Expandable,
// //         turn: Color::White,
// //         move_num: 12.,
// //         value: Some(-100), //TODO make value not an option
// //         repetition_detector: repetition_detector,
// //         max_score: None,
// //         min_score: None,
// //         n: 1.,
// //         q: 0.,
// //     };
// //     let settings = Settings::lib_test_default();
// //     let seed = 1;
// //     let mut delta = 0.;
// //     let n = 50;
// //     for _i in 0..n as u32 {
// //         delta = node.iteration(
// //             &mut game.clone(),
// //             &mut seeded_rng(seed),
// //             &mut stats,
// //             &settings,
// //         );
// //         if node.outcome.is_some() {
// //             break;
// //         }
// //     }
// //     print_tree(&node, &settings);
// //     assert_eq!(None, node.outcome);
// //     assert_eq!(n as f32 + 1., node.n);
// //     assert_eq!(Color::White, node.turn);
// //     assert_eq!(NodeState::FullyExpanded, node.state);
// //     assert_eq!(Some(0), node.min_score);
// //     assert_eq!(None, node.max_score);
// //     let best_move = play::find_best_move(node, &game, &mut stats, &settings).unwrap();
// //     assert_eq!(
// //         Move::Normal {
// //             role: Role::King,
// //             from: Square::F2,
// //             capture: None,
// //             to: Square::F1,
// //             promotion: None
// //         },
// //         best_move.action.unwrap()
// //     );
// // }
// //
// // #[test]
// // fn test_iteration_mate_in_2_1_choice() {
// //     let mut stats: RunStats = Default::default();
// //     let settings = Settings::lib_test_default();
// //     let (node, score) =
// //         test_iteration_all_children("4q3/8/8/8/8/3k4/8/3K4 b - - 0 1", &mut stats, &settings);
// //     println!("{}", stats);
// //     print_tree(&node, &settings);
// //     assert_eq!(-1., score);
// //     assert_eq!(
// //         Outcome::Decisive {
// //             winner: Color::Black
// //         },
// //         node.outcome.unwrap()
// //     );
// //     assert!(stats.nodes_created < 1000);
// // }
// //
// // #[test]
// // fn test_iteration_mate_in_2_2_choices() {
// //     let mut stats: RunStats = Default::default();
// //     let settings = Settings::lib_test_default();
// //     let (node, score) = test_iteration_all_children(
// //         "8/5Q2/1pkq2n1/pB2p3/4P3/1P2K3/2P5/8 b - - 1 1",
// //         &mut stats,
// //         &settings,
// //         );
// //     println!("{}", stats);
// //     print_tree(&node, &settings);
// //     assert_eq!(1., score);
// //     assert_eq!(
// //         Outcome::Decisive {
// //             winner: Color::White
// //         },
// //         node.outcome.unwrap()
// //     );
// //     assert!(stats.nodes_created < 60);
// // }
//
// fn test_single_iteration(fen_str: &'static str, stats: &mut RunStats) -> (TreeNode, f32) {
//     test_single_iteration_with_repetitions(fen_str, vec![], stats)
// }
//
// fn test_single_iteration_with_repetitions(
//     fen_str: &'static str,
//     repetitions: Vec<&'static str>,
//     stats: &mut RunStats,
// ) -> (TreeNode, f32) {
//     let settings = Settings::test_iteration_default();
//     let position = parse_fen(fen_str);
//     print_emojified(&position.board());
//     let mut rng = seeded_rng(settings.starting_seed);
//     let mut node = TreeNode::new_root(&position, 1.);
//     for fen in repetitions {
//         let repeated_pos = parse_fen(fen);
//         node.repetition_detector.record_and_check(&repeated_pos);
//     }
//     let mut delta = 0.;
//     stats.start_timer();
//     delta = node.iteration(&mut position.clone(), &mut rng, stats, &settings);
//     stats.stop_timer();
//     print_tree(&node, &settings);
//     println!("{}", stats);
//     (node, delta)
// }
//
// fn test_iteration_until_outcome(fen_str: &'static str, stats: &mut RunStats) -> (TreeNode, f32) {
//     START.call_once(|| {
//         logger::init();
//     });
//     let settings = Settings::test_iteration_default();
//     let max_iterations = match settings.search_type {
//         SearchType::Iterations(max_iterations) => max_iterations,
//         _ => panic!("invalid test iteration setup"),
//     };
//     let position = parse_fen(fen_str);
//     print_emojified(&position.board());
//     let mut rng = seeded_rng(settings.starting_seed);
//     let mut node = TreeNode::new_root(&position, 1.);
//     let mut delta = 0.;
//     stats.start_timer();
//     for _i in 0..max_iterations {
//         delta = node.iteration(&mut position.clone(), &mut rng, stats, &settings);
//         if node.has_outcome() {
//             break;
//         }
//     }
//     stats.stop_timer();
//     assert!(node.has_outcome());
//     print_tree(&node, &settings);
//     println!("{}", stats);
//     (node, delta)
// }
