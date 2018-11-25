use eval::*;
use shakmaty::*;
use std::i16;
use std::ops::Not;

pub const MAX_HALFMOVES: u32 = 101;
pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub type Reward = i16;
pub const MAX_REWARD: Reward = i16::MAX;
pub const MIN_REWARD: Reward = i16::MIN + 1; // for some reason, min is -32768 but max is 32767. The + 1 prevents overflows when we flip signs

pub trait Game: Clone {
    fn allowed_actions(&self) -> Vec<Move>;
    fn make_move(&mut self, action: &Move);
    fn play_safe(&mut self, &Move);
    fn display_move_num(&self) -> String;
    fn clone_and_play(&self, action: &Move) -> Chess;
    fn color_relative_reward(&self) -> i16;
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

    fn display_move_num(&self) -> String {
        format!(
            "{}{}",
            if self.turn() == Color::Black {
                ".."
            } else {
                ""
            },
            self.fullmoves()
        )
    }

    fn clone_and_play(&self, action: &Move) -> Chess {
        let mut new_position = self.clone();
        new_position.make_move(action);
        new_position
    }

    fn color_relative_reward(&self) -> i16 {
        self.turn().not().coefficient() * self.reward()
    }
}

impl HasReward for Chess {
    fn reward(&self) -> i16 {
        match self.outcome() {
            Some(o) => o.reward(),
            None => self.board().reward(),
        }
    }
}

impl HasReward for Outcome {
    fn reward(&self) -> Reward {
        match self {
            Outcome::Decisive {
                winner: Color::Black,
            } => MIN_REWARD,
            Outcome::Decisive {
                winner: Color::White,
            } => MAX_REWARD,
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
