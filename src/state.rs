use mcts::*;
use settings::*;
use setup::*;
use shakmaty::*;
use stats::*;

#[derive(Default)]
pub struct State {
    root: TreeNode,
    position: Chess,
    game_stats: RunStats,
    settings: Settings,
}

impl State {
    // TODO starting position needs to be registered with repetition detector
    pub fn from_fen(fen_str: &str) -> Result<State, String> {
        parse_fen_input(fen_str).map(|f| State {
            position: f,
            ..Default::default()
        })
    }
}
