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
    pub position: Chess,
    pub time_remaining: Option<TimeRemaining>,
    pub opponent_time_remaining: Option<Duration>,
}

impl State {
    // TODO starting position needs to be registered with repetition detector
    pub fn from_position(position: Chess) -> State {
        State {
            position: position.clone(),
            root: TreeNode::new_root(&position, 0.5),
            ..Default::default()
        }
    }

    pub fn search(self, stats: &mut RunStats, settings: &Settings) -> State {
        let position = self.position.clone();
        let time_remaining = self.time_remaining.clone();
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        State {
            root: search_with_strategy(self, stats, settings),
            position: position,
            time_remaining: time_remaining,
            opponent_time_remaining: opponent_time_remaining,
        }
    }

    pub fn make_best_move(self) -> State {
        let mut new_position = self.position.clone();
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        let time_remaining = self
            .time_remaining
            .clone()
            .map(|t| t.recalculate_from_now());
        let new_root = self.best_child_node();
        new_position.make_move(&new_root.action.clone().unwrap());
        State {
            root: new_root,
            position: new_position,
            opponent_time_remaining: opponent_time_remaining,
            time_remaining: time_remaining,
        }
    }

    pub fn best_child_node(self) -> TreeNode {
        // TODO try the equation from the MCTS-Solver paper
        self.root
            .children
            .into_iter()
            .max_by(|c1, c2| {
                c1.color_relative_minimax()
                    .cmp(&c2.color_relative_minimax())
            })
            .unwrap()
    }
    pub fn make_user_move(self, action: &Move) -> State {
        let mut new_position = self.position.clone();
        new_position.make_move(action);
        let prev_move_num = self.root.move_num;
        let time_remaining = self.time_remaining.clone();
        let opponent_time_remaining = self.opponent_time_remaining.clone();
        let new_root = self.find_child_by_action(action);
        State {
            root: new_root.unwrap_or_else(|| {
                error!("child by action not found");
                TreeNode::new_root(&new_position, prev_move_num + 0.5)
            }),
            position: new_position,
            time_remaining: time_remaining,
            opponent_time_remaining: opponent_time_remaining,
            ..Default::default()
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

    pub fn has_outcome(&self) -> bool {
        self.root.has_outcome()
    }

    pub fn is_decisive(&self) -> bool {
        self.root.is_decisive()
    }

    pub fn last_action(&self) -> Move {
        self.root.action.clone().unwrap()
    }

    pub fn turn(&self) -> Color {
        self.position.turn()
    }

    pub fn q(&self) -> f32 {
        self.root.q
    }

    pub fn ply(&self) -> f32 {
        self.position.fullmoves() as f32 / 2.
    }

    pub fn game_over(&self) -> bool {
        self.position.is_game_over()
    }

    pub fn score(&self) -> i16 {
        self.root.score()
    }
}
