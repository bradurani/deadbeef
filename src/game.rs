use eval::*;
use shakmaty::*;

pub const MAX_HALFMOVES: u32 = 101;
pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub type Reward = i16;

pub trait Game: Clone {
    fn allowed_actions(&self) -> Vec<Move>;
    fn make_move(&mut self, action: &Move);
    fn play_safe(&mut self, &Move);
    fn ply(&self) -> f32;
    fn has_outcome(&self) -> bool;
}

impl Game for Chess {
    // TODO this would probably be faster if didn't create the whoe list up front,
    // but rather iterated through it
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
        if cfg!(debug_assertions) {
            self.play_safe(&action);
        } else {
            self.play_unchecked(&action);
        }
    }

    fn play_safe(&mut self, m: &Move) {
        if self.is_legal(m) {
            self.play_unchecked(m)
        } else {
            panic!("Illegal Move Play\n{}", m);
        }
    }

    fn ply(&self) -> f32 {
        self.fullmoves() as f32 / 2.
    }

    fn has_outcome(&self) -> bool {
        self.outcome().is_some()
    }
}

pub trait IsOutcome {
    fn is_decisive(&self) -> bool;
    fn is_draw(&self) -> bool;
    fn reward(&self) -> Reward;
}

impl IsOutcome for Outcome {
    fn is_decisive(&self) -> bool {
        match self {
            Outcome::Decisive { winner: _ } => true,
            _ => false,
        }
    }

    fn is_draw(&self) -> bool {
        match self {
            Outcome::Draw => true,
            _ => false,
        }
    }

    fn reward(&self) -> Reward {
        match self {
            Outcome::Decisive {
                winner: Color::Black,
            } => i16::min_value(),
            Outcome::Decisive {
                winner: Color::White,
            } => i16::max_value(),
            Outcome::Draw => 0,
        }
    }
}

pub trait Coefficient {
    fn coefficient(&self) -> i16;
}

impl Coefficient for Color {
    fn coefficient(&self) -> i16 {
        match &self {
            Color::Black => -1,
            Color::White => 1,
        }
    }
}
