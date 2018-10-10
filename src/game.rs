use shakmaty::*;

/// A `Game` represets a game state.
///
/// It is important that the game behaves fully deterministic,
/// e.g. it has to produce the same game sequences
pub trait Game: Clone {
    /// Return a list with all allowed actions given the current game state.
    fn allowed_actions(&self) -> Vec<Move>;

    /// Change the current game state according to the given action.
    fn make_move(&mut self, action: &Move);

    /// Reward for the player when reaching the current game state.
    fn reward(&self) -> f32;
}

impl Game for Chess {
    fn allowed_actions(&self) -> Vec<Move> {
        match &self.is_game_over() {
            true => Vec::new(),
            false => {
                let mut moves = MoveList::new();
                self.legal_moves(&mut moves);
                moves.to_vec()
            }
        }
    }

    fn make_move(&mut self, action: &Move) {
        self.play_unchecked(&action);
        // self.play_safe(&action)
        // TODO add safe option for testing
    }

    fn reward(&self) -> f32 {
        let outcome = self.outcome();
        match outcome {
            Some(Outcome::Decisive {
                winner: Color::Black,
            }) => -1.0,
            Some(Outcome::Decisive {
                winner: Color::White,
            }) => 1.0,
            Some(Outcome::Draw) => 0.0,
            None => 0.0,
        }
    }
}

pub trait Coefficient {
    fn coefficient(&self) -> f32;
}
impl Coefficient for Color {
    fn coefficient(&self) -> f32 {
        match &self {
            Color::Black => -1.,
            Color::White => 1.,
        }
    }
}
