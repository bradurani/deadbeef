use display::*;
use game::*;
use search_strategy::*;
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
        info!("\n{:?}", settings);
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
        info!("=========  user ==========");
        let action = parse_uci_input(uci_str, &self.position())?;
        self.change_state(|s| s.make_move(&action));
        debug_print_tree(&self.state.root, &self.settings);
        info!("{}", self);
        info!("==========================");
        Ok(action)
    }

    pub fn make_engine_move(&mut self) -> Result<Move, String> {
        info!("++++++++++ engine ++++++++++");
        let search_type = &self.settings.search_type.clone();
        self.search(search_type)?;
        let best_move = self.best_move();
        self.change_state(|s| s.make_move(&best_move));
        info!("{}", self);
        info!("+++++++++++++++++++++++++++");
        Ok(best_move)
    }

    pub fn test_search(&mut self, search_type: &SearchType) -> Move {
        self.search(search_type)
            .expect("could not perform test search");
        self.best_move()
    }

    pub fn position(&self) -> Chess {
        self.state.position()
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

    pub fn is_game_over(&self) -> bool {
        self.state.is_game_over()
    }

    pub fn is_decisive(&self) -> bool {
        self.state.is_decisive()
    }

    pub fn is_checkmate(&self) -> bool {
        self.state.is_checkmate()
    }

    pub fn print_tree(&self) -> Result<(), String> {
        self.print_subtree(vec![])
    }

    pub fn best_move(&self) -> Move {
        self.state.best_move()
    }

    pub fn minimax(&self) -> Reward {
        self.state.minimax()
    }

    pub fn print_subtree(&self, action_uci_strs: Vec<&str>) -> Result<(), String> {
        let mut root = &self.state.root;
        let mut position = self.state.position();
        for uci_str in action_uci_strs {
            let action = parse_uci_input(uci_str, &position)?;
            root = root
                .children
                .iter()
                .find(|c| c.action.clone().unwrap() == action)
                .ok_or("could not find child")?;
            position = position.play(&action).map_err(|e| e.to_string())?;
        }
        info_print_tree(&root, &self.settings);
        Ok(())
    }

    pub fn search_with_settings(&mut self) -> Result<(), String> {
        let search_type = &self.settings.search_type.clone();
        self.search(search_type)
    }

    pub fn search(&mut self, search_type: &SearchType) -> Result<(), String> {
        if self.is_game_over() {
            return Err("game is over".to_string());
        }
        let mut move_run_stats: RunStats = Default::default();
        let settings = self.settings.clone();
        self.change_state(|s| s.search(search_type.clone(), &mut move_run_stats, &settings));
        debug_print_tree(&self.state.root, &self.settings);
        info!("{}", move_run_stats);
        self.game_stats.add(&move_run_stats);
        Ok(())
    }

    fn change_state<F: FnMut(State) -> State>(&mut self, mut f: F) {
        let prev_state = mem::replace(&mut self.state, Default::default());
        self.previous_position = prev_state.position();
        self.state = f(prev_state);
    }
}
