use settings::*;
use shakmaty::fen::*;
use shakmaty::*;
use state::*;
use stats::*;

#[derive(Default)]
pub struct Engine {
    pub state: State,
    pub game_stats: RunStats,
    pub settings: Settings,
}

impl Engine {
    pub fn set_board(&mut self, fen_str: &str) -> Result<(), String> {
        State::from_fen(fen_str).map(|state| {
            self.state = state;
            self.game_stats = Default::default();
        })
    }

    pub fn reset(&mut self) {
        self.set_board(Fen::STARTING_POSITION).unwrap();
    }

    pub fn make_move(&mut self) -> Move {
        self.search();
        self.state = self.state.make_best_move();
        self.state.last_action()
    }

    fn search(&mut self) {
        if self.state.has_outcome() {
            return;
        }
        let move_run_stats: RunStats = Default::default();
        self.state = self.state.search(&mut move_run_stats, &self.settings);
        self.game_stats.add(&move_run_stats);
    }
}
