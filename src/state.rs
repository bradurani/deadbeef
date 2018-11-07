use game::*;
use mcts::*;
use search_strategy::*;
use settings::*;
use setup::*;
use shakmaty::*;
use stats::*;

#[derive(Default)]
pub struct State {
    pub root: TreeNode,
    pub position: Chess,
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
        let new_root = search_with_strategy(self, stats, settings);
        State {
            root: new_root,
            position: position,
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
