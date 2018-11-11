use shakmaty::*;

pub const MAX_HALFMOVES: u32 = 101;
pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub trait Game: Clone {
    fn allowed_actions(&self) -> Vec<Move>;
    fn make_move(&mut self, action: &Move);
    fn play_safe(&mut self, &Move);
    fn ply(&self) -> f32;
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
}

pub trait IsOutcome {
    fn is_decisive(&self) -> bool;
    fn is_draw(&self) -> bool;
    fn reward(&self) -> f32;
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

    fn reward(&self) -> f32 {
        match self {
            Outcome::Decisive {
                winner: Color::Black,
            } => -1.0,
            Outcome::Decisive {
                winner: Color::White,
            } => 1.0,
            Outcome::Draw => 0.0,
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
