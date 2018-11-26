use game::*;
use log::*;
use search_strategy::*;
use settings::*;
use shakmaty::*;
use stats::*;
use std::time::Duration;
use time_remaining::*;
use tree_node::*;

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

    pub fn search(
        self,
        search_type: SearchType,
        stats: &mut RunStats,
        settings: &Settings,
    ) -> State {
        let time_remaining = self.time_remaining.clone();
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        State {
            root: search_with_search_type(self, search_type, stats, settings),
            time_remaining: time_remaining,
            opponent_time_remaining: opponent_time_remaining,
        }
    }

    pub fn best_move(&self) -> Move {
        // TODO try the equation from the MCTS-Solver paper
        self.root
            .children
            .iter()
            .max_by(|c1, c2| {
                if c1.best_child_sort_use_minimax() || c2.best_child_sort_use_minimax() {
                    c1.best_child_sort_minimax()
                        .cmp(&c2.best_child_sort_minimax())
                } else {
                    c1.best_child_sort_n()
                        .partial_cmp(&c2.best_child_sort_n())
                        .unwrap()
                }
            })
            .and_then(|c| c.action.clone())
            .expect("no best child to choose from")
    }

    pub fn make_move(self, action: &Move) -> State {
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

    pub fn is_checkmate(&self) -> bool {
        self.root.is_checkmate()
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

    pub fn display_move_num(&self) -> String {
        self.root.display_move_num()
    }

    pub fn position(&self) -> Chess {
        self.root.position.clone()
    }

    pub fn is_game_over(&self) -> bool {
        self.root.is_game_over()
    }

    pub fn minimax(&self) -> Reward {
        self.root.minimax
    }

    pub fn record_test_repetitions(&mut self, repetition_positions: Vec<Chess>) {
        for r in repetition_positions {
            self.root.repetition_detector.record(&r)
        }
    }
}
