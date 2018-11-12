use display::*;
use game::*;
use settings::*;
use setup::*;
use shakmaty::*;
use state::*;
use stats::*;
use std::mem;
use std::time::Duration;

#[derive(Default)]
pub struct Engine {
    pub state: State,
    pub color: Option<Color>,
    pub previous_position: Chess, // we need this after making a move so we can generate Uci
    pub game_stats: RunStats,
    pub settings: Settings,
}

impl Engine {
    pub fn new(settings: Settings) -> Engine {
        Engine {
            settings: settings,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.set_board(STARTING_POSITION).unwrap();
    }

    pub fn set_board(&mut self, fen_str: &str) -> Result<(), String> {
        parse_fen_input(fen_str).map(|position| {
            self.state = State::from_position(position);
            self.game_stats = Default::default();
            info!("{}", self);
        })
    }

    pub fn make_user_move(&mut self, uci_str: &str) -> Result<Move, String> {
        let action = parse_uci_input(uci_str, self.position())?;
        self.change_state(|s| s.make_user_move(&action));
        Ok(action)
    }

    pub fn make_engine_move(&mut self) -> Move {
        self.search();
        print_tree(&self.state.root, &self.settings);
        self.change_state(|s| s.make_best_move());
        info!("{}", self);
        self.state.last_action()
    }

    pub fn position(&self) -> Chess {
        self.state.position.clone()
    }

    pub fn set_time_remaining_cs(&mut self, remaining_cs: u64) {
        let remaining = Duration::from_millis(remaining_cs * 10);
        self.change_state(|s| s.set_time_remaining(remaining))
    }

    pub fn set_opponent_time_remaining_cs(&mut self, remaining_cs: u64) {
        let remaining = Duration::from_millis(remaining_cs * 10);
        self.change_state(|s| s.set_opponent_time_remaining(remaining));
    }

    pub fn set_show_thinking(&mut self, show_thinking: bool) {
        self.settings.show_thinking = show_thinking;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn print_tree_from_child(&mut self, uci_str: &str) -> Result<(), String> {
        let action = parse_uci_input(uci_str, self.state.position.clone())?;
        let child = &self
            .state
            .root
            .children
            .iter()
            .find(|c| c.action.clone().unwrap() == action)
            .ok_or("could not find child")?;
        print_tree(child, &self.settings);
        Ok(())
    }

    pub fn print_tree(&self) {
        print_tree(&self.state.root, &self.settings);
    }

    pub fn game_over(&self) -> bool {
        return self.state.game_over();
    }

    pub fn search(&mut self) {
        if self.state.has_outcome() {
            return;
        }
        let mut move_run_stats: RunStats = Default::default();
        let settings = self.settings.clone();
        self.change_state(|s| s.search(&mut move_run_stats, &settings));
        self.game_stats.add(&move_run_stats);
    }

    fn change_state<F: FnMut(State) -> State>(&mut self, mut f: F) {
        let prev_state = mem::replace(&mut self.state, Default::default());
        self.previous_position = prev_state.position.clone();
        self.state = f(prev_state);
    }
}
