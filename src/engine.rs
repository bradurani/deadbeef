use settings::*;
use setup::*;
use shakmaty::fen::*;
use shakmaty::*;
use state::*;
use stats::*;
use std::mem;
use std::time::Duration;

#[derive(Default)]
pub struct Engine {
    pub state: State,
    pub previous_position: Chess, // we need to this after making a move so we can generate Uci
    pub game_stats: RunStats,
    pub settings: Settings,
}

pub fn new(settings: Settings) -> Engine {
    Engine {
        settings: settings,
        ..Default::default()
    }
}

impl Engine {
    pub fn reset(&mut self) {
        self.set_board(Fen::STARTING_POSITION).unwrap();
    }

    pub fn set_board(&mut self, fen_str: &str) -> Result<(), String> {
        State::from_fen(fen_str).map(|state| {
            self.state = state;
            self.game_stats = Default::default();
        })
    }

    pub fn make_user_move(&mut self, uci_str: &str) -> Result<Move, String> {
        let action = parse_uci_input(uci_str, self.position())?;
        let old_state = mem::replace(&mut self.state, Default::default());
        self.previous_position = old_state.position.clone();
        self.state = old_state.make_user_move(&action);
        Ok(action)
    }

    pub fn make_engine_move(&mut self) -> Move {
        self.search();
        let old_state = mem::replace(&mut self.state, Default::default());
        self.previous_position = old_state.position.clone();
        self.state = old_state.make_best_move();
        self.state.last_action()
    }

    pub fn position(&self) -> Chess {
        self.state.position.clone()
    }

    pub fn set_time_remaining_cs(&mut self, remaining_cs: u64) {
        let remaining = Duration::from_millis(remaining_cs * 10);
        let old_state = mem::replace(&mut self.state, Default::default());
        self.state = old_state.set_time_remaining(remaining);
    }

    pub fn set_opponent_time_remaining_cs(&mut self, remaining_cs: u64) {
        let remaining = Duration::from_millis(remaining_cs * 10);
        let old_state = mem::replace(&mut self.state, Default::default());
        self.state = old_state.set_opponent_time_remaining(remaining);
    }

    pub fn set_show_thinking(&mut self, show_thinking: bool) {
        self.settings.show_thinking = show_thinking;
    }

    fn search(&mut self) {
        if self.state.has_outcome() {
            return;
        }
        let mut move_run_stats: RunStats = Default::default();
        let old_state = mem::replace(&mut self.state, Default::default());
        self.state = old_state.search(&mut move_run_stats, &self.settings);
        self.game_stats.add(&move_run_stats);
    }
}
