use game::*;
use log::*;
use mcts::*;
use search_strategy::*;
use settings::*;
use shakmaty::*;
use stats::*;
use std::time::Duration;
use time_remaining::*;

#[derive(Default)]
pub struct State {
    pub root: TreeNode,
    pub time_remaining: Option<TimeRemaining>,
    pub opponent_time_remaining: Option<Duration>,
}

impl State {
    pub fn from_position(position: Chess) -> State {
        State {
            root: TreeNode::new_root(position.clone()),
            ..Default::default()
        }
    }

    pub fn search(self, stats: &mut RunStats, settings: &Settings) -> State {
        let time_remaining = self.time_remaining.clone();
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        State {
            root: search_with_strategy(self, stats, settings),
            time_remaining: time_remaining,
            opponent_time_remaining: opponent_time_remaining,
        }
    }

    pub fn make_best_move(self) -> State {
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        let time_remaining = self
            .time_remaining
            .clone()
            .map(|t| t.recalculate_from_now());
        let new_root = self.best_child_node();
        State {
            root: new_root,
            opponent_time_remaining,
            time_remaining,
        }
    }

    pub fn best_child_node(self) -> TreeNode {
        // TODO try the equation from the MCTS-Solver paper
        self.root
            .children
            .into_iter()
            .max_by(|c1, c2| {
                let c1_value = c1.best_child_sort_value();
                let c2_value = c2.best_child_sort_value();
                if c1_value == c2_value {
                    // if n is equal, fall back to minimax
                    // occurs in shallow search trees
                    c1.color_relative_minimax()
                        .cmp(&c2.color_relative_minimax())
                } else {
                    c1.best_child_sort_value().cmp(&c2.best_child_sort_value())
                }
            })
            .expect("no children to choose from")
    }

    pub fn make_user_move(self, action: &Move) -> State {
        let time_remaining = self.time_remaining.clone();
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        let mut position = self.position();
        let new_root = self.find_child_by_action(action);
        State {
            root: new_root.unwrap_or_else(|| {
                warn!("child by action not found");
                position.play_safe(action);
                TreeNode::new_root(position)
            }),
            time_remaining,
            opponent_time_remaining,
        }
    }

    pub fn find_child_by_action(self, action: &Move) -> Option<TreeNode> {
        let found = self
            .root
            .children
            .into_iter()
            .find(|c| c.action.clone().unwrap() == *action);
        found
    }

    pub fn set_time_remaining(self, remaining: Duration) -> State {
        State {
            time_remaining: Some(TimeRemaining::start(remaining)),
            ..self
        }
    }

    pub fn set_opponent_time_remaining(self, remaining: Duration) -> State {
        State {
            opponent_time_remaining: Some(remaining),
            ..self
        }
    }

    pub fn is_decisive(&self) -> bool {
        self.root.is_decisive()
    }

    pub fn last_action(&self) -> Move {
        self.root.action.clone().unwrap()
    }

    pub fn turn(&self) -> Color {
        self.root.turn()
    }

    pub fn q(&self) -> f32 {
        self.root.q
    }

    pub fn move_num(&self) -> f32 {
        self.root.move_num()
    }

    pub fn game_over(&self) -> bool {
        self.root.is_game_over()
    }

    pub fn position(&self) -> Chess {
        self.root.position.clone()
    }

    pub fn is_game_over(&self) -> bool {
        self.root.is_game_over()
    }
}
