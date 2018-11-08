use game::*;
use mcts::*;
use search_strategy::*;
use settings::*;
use setup::*;
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
    pub fn from_fen(fen_str: &str) -> Result<State, String> {
        parse_fen_input(fen_str).map(|f| State {
            position: f,
            ..Default::default()
        })
    }

    pub fn search(self, stats: &mut RunStats, settings: &Settings) -> State {
        let position = self.position.clone();
        let time_remaining = self.time_remaining.clone();
        State {
            root: search_with_strategy(self, stats, settings),
            position: position,
            time_remaining: time_remaining,
            ..Default::default()
        }
    }

    pub fn make_best_move(self) -> State {
        let mut new_position = self.position.clone();
        let new_root = self.best_child_node();

        new_position.make_move(&new_root.action.unwrap());
        State {
            root: new_root,
            position: new_position,
            ..Default::default()
        }
    }

    pub fn best_child_node(self) -> TreeNode {
        // TODO try the equation from the MCTS-Solver paper
        self.root
            .children
            .into_iter()
            .max_by(|n1, n2| {
                n1.color_relative_score()
                    .partial_cmp(&n2.color_relative_score())
                    .unwrap()
            })
            .unwrap()
    }

    pub fn make_user_move(self, action: &Move) -> State {
        let mut new_position = self.position.clone();
        new_position.make_move(action);
        let prev_move_num = self.root.move_num;
        let new_root = self.find_child_by_action(action);
        State {
            root: new_root.unwrap_or_else(|| {
                // eprintln!("child by action not found");
                TreeNode::new_root(&new_position, prev_move_num + 0.5)
            }),
            position: new_position,
            ..Default::default()
        }
    }

    pub fn find_child_by_action(self, action: &Move) -> Option<TreeNode> {
        eprintln!("finding......");
        let found = self
            .root
            .children
            .into_iter()
            .find(|c| c.action.unwrap() == *action);
        eprintln!("found");
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

    pub fn last_action(&self) -> Move {
        self.root.action.unwrap()
    }
}

pub struct StateSnapshot {
    stats: RunStats,
}
