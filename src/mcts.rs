use eval::*;
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
    fn expand(&mut self, rng: &mut SmallRng, stats: &mut RunStats, settings: &Settings) -> f32;
    fn actions_with_no_children(&self) -> Vec<Move>;
    fn check_fully_expanded(&mut self);
    fn normalized_value(&self) -> f32;
    fn set_minimax_based_on_children(&mut self);
    fn generate_missing_children(&mut self);
}

impl MCTS for TreeNode {
    fn iteration(&mut self, rng: &mut SmallRng, stats: &mut RunStats, settings: &Settings) -> f32 {
        stats.iterations += 1;
        debug_assert!(self.position.halfmoves() <= MAX_HALFMOVES);
        let normalized_value: f32 = match self.state {
            NodeState::FullyExpanded => {
                let normalized_value = {
                    let child = best_child(self, settings);
                    child.iteration(rng, stats, settings)
                };
                self.set_minimax_based_on_children();
                normalized_value
            }
            NodeState::Expandable => {
                let normalized_value = self.expand(rng, stats, &settings);
                normalized_value
            }
            NodeState::Empty => {
                self.value = playout(self.position.clone(), stats, settings);
                self.minimax = self.value;
                self.state = NodeState::Expandable;
                stats.nodes_created += 1;
                self.normalized_value()
            }
            NodeState::LeafNode => {
                panic!("IMPOSSIBLE LeafNode");
            }
        };
        self.check_fully_expanded();
        self.n += 1;
        self.q += normalized_value;
        normalized_value
    }

    fn expand(&mut self, rng: &mut SmallRng, stats: &mut RunStats, settings: &Settings) -> f32 {
        //TODO is this actually better than random?
        let candidate_actions = self.actions_with_no_children();
        // let action = choose_random(rng, &candidate_actions);
        let action = candidate_actions
            .iter()
            .max_by(|a, b| {
                let position_a = self.position.clone_and_play(a);
                let position_b = self.position.clone_and_play(b);
                position_a.board().value().cmp(&position_b.board().value())
            })
            .expect("no children to choose best from");

        let mut child = TreeNode::new_empty_child(action.clone(), self);
        if child.is_game_over() {
            child.value = child.outcome().expect("no outcome for child").reward();
            child.state = NodeState::LeafNode;
            stats.leaf_nodes += 1;
        } else {
            child.value = playout(self.position.clone(), stats, settings);
            child.state = NodeState::Expandable;
        }

        let normalized_value = child.normalized_value();
        child.q = normalized_value;
        child.minimax = child.value;
        child.n += 1;
        self.children.push(child);
        stats.nodes_created += 1;
        normalized_value
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

    fn check_fully_expanded(&mut self) {
        if [NodeState::Empty, NodeState::Expandable].contains(&self.state)
            && self.actions_with_no_children().is_empty()
        {
            self.state = NodeState::FullyExpanded;
            self.set_minimax_based_on_children();
        }
    }

    fn normalized_value(&self) -> f32 {
        (self.value as f32 / 9590.).min(1.) // (8 * 929) + (2 * 479) + (2 * 320) + (2 * 280)
                                            // TODO test 8 queen positions and other extremes
    }

    fn set_minimax_based_on_children(&mut self) {
        if self.state != NodeState::FullyExpanded && self.state != NodeState::LeafNode {
            return;
        }
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
        if self.has_winning_child() || self.all_children_leaf_nodes() {
            self.state = NodeState::LeafNode;
            self.minimax -= self.turn().coefficient();
        }
    }

    fn generate_missing_children(&mut self) {
        // TODO, set the leaf node state in the TreeNode constructors
        for action in self.actions_with_no_children() {
            let node = TreeNode::new_empty_child(action, &self);
            self.children.push(node);
        }
    }
}
