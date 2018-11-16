// pub fn expand(
//     &mut self,
//     game: &mut Chess,
//     candidate_actions: Vec<Move>,
//     rng: &mut SmallRng,
//     thread_run_stats: &mut RunStats,
// ) -> &mut TreeNode {
//     let action = choose_random(rng, &candidate_actions);
//     game.make_move(action);
//     let new_rep = self.repetition_detector.clone();
//     let new_node = TreeNode::new(
//         Some(action.clone()),
//         self.turn.not(),
//         self.move_num + 0.5,
//         game.board().value(),
//         new_rep,
//     );
//     self.children.push(new_node);
//     thread_run_stats.nodes_created += 1;
//     self.children.last_mut().unwrap()
// }
