use game::*;
use playout::*;
use rand::rngs::SmallRng;
use settings::*;
use shakmaty::*;
use stats::*;
use std::f32;
use tree_node::*;
use uct::*;

pub trait MCTS {
    fn iteration(&mut self, rng: &mut SmallRng, stats: &mut RunStats, settings: &Settings) -> f32;
    fn expand(&self, candidate_actions: &Vec<Move>) -> TreeNode;
    fn actions_with_no_children(&self) -> Vec<Move>;
    fn update_based_on_children(&mut self);
    fn normalized_value(&self) -> f32;
    fn set_minimax_based_on_children(&mut self);
    fn generate_missing_children(&mut self, stats: &mut RunStats);
}

impl MCTS for TreeNode {
    fn iteration(&mut self, rng: &mut SmallRng, stats: &mut RunStats, settings: &Settings) -> f32 {
        stats.iterations += 1;
        debug_assert!(self.position.halfmoves() <= MAX_HALFMOVES);
        let normalized_value: f32 = match self.state {
            NodeState::FullyExpanded => {
                let normalized_value = {
                    let child = most_interesting_child(self, settings);
                    stats.increase_mcts_depth();
                    let normalized_value = child.iteration(rng, stats, settings);
                    stats.decrease_mcts_depth();
                    normalized_value
                };
                normalized_value
            }
            NodeState::Expandable => {
                let candidate_actions = self.actions_with_no_children();
                let mut child = self.expand(&candidate_actions);
                self.children.push(child);
                stats.increase_mcts_depth();
                let normalized_value = self
                    .children
                    .last_mut()
                    .unwrap()
                    .iteration(rng, stats, settings); // starts Empty, so now do playout
                stats.decrease_mcts_depth();
                if candidate_actions.len() == 1 {
                    self.state = NodeState::FullyExpanded;
                }
                normalized_value
            }
            NodeState::Empty => {
                if self.is_game_over() {
                    self.value = self.outcome().unwrap().reward();
                    self.state = NodeState::LeafNode;
                    stats.leaf_nodes += 1;
                } else {
                    self.value = playout(self.position.clone(), stats, settings);
                    self.state = NodeState::Expandable;
                }
                self.minimax = self.value;
                stats.nodes_created += 1;
                self.normalized_value()
            }
            NodeState::LeafNode => {
                panic!("IMPOSSIBLE LeafNode");
            }
            NodeState::FullySearched => {
                panic!("IMPOSSIBLE FullySearched");
            }
        };
        self.n += 1;
        self.q += normalized_value;
        self.update_based_on_children();
        normalized_value
    }

    fn expand(&self, candidate_actions: &Vec<Move>) -> TreeNode {
        //TODO is this actually better than random?
        let action = candidate_actions
            .iter()
            .max_by(|a, b| {
                let position_a = self.position.clone_and_play(a);
                let position_b = self.position.clone_and_play(b);
                position_a
                    .color_relative_value()
                    .cmp(&position_b.color_relative_value())
            })
            .expect("no children to expand");

        TreeNode::new_empty_child(action.clone(), self)
    }

    fn actions_with_no_children(&self) -> Vec<Move> {
        debug_assert!(self.position.legals().len() > 0);
        let child_actions: Vec<Move> = self
            .children
            .iter()
            .map(|c| c.action.clone().unwrap())
            .collect();
        self.position
            .legals()
            .into_iter()
            .filter(|a| !child_actions.contains(a))
            .collect()
    }

    fn update_based_on_children(&mut self) {
        if self.state == NodeState::FullyExpanded {
            self.set_minimax_based_on_children();
            if self.children.iter().all(|c| !c.searchable()) {
                self.state = NodeState::FullySearched
            }
        }
    }

    fn normalized_value(&self) -> f32 {
        (self.value as f32 / 9590.).min(1.) // (8 * 929) + (2 * 479) + (2 * 320) + (2 * 280)
                                            // TODO test 8 queen positions and other extremes
    }

    fn set_minimax_based_on_children(&mut self) {
        assert!(self.state == NodeState::FullyExpanded);
        let new_minimax = self
            .children
            .iter()
            .map(|c| c.minimax)
            .max_by(|v1, v2| {
                let relative_v1 = v1 * self.turn().coefficient();
                let relative_v2 = v2 * self.turn().coefficient();
                relative_v1.cmp(&relative_v2)
            })
            .expect("no children to choose minimax from");
        self.minimax = new_minimax;
    }

    fn generate_missing_children(&mut self, _stats: &mut RunStats) {
        for action in self.actions_with_no_children() {
            let mut child = TreeNode::new_empty_child(action, &self);
            self.children.push(child);
        }
    }
}
