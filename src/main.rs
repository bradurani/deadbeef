#![feature(duration_as_u128)]

extern crate rand;
extern crate shakmaty;
extern crate matches;

use rand::{sample, thread_rng};
use shakmaty::*;
use mcts::{Game, GameAction, MCTS};
use std::time::{Instant};

pub mod pgn;
pub mod mcts;
pub mod utils;

impl mcts::Game<Move> for Chess {
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

    fn make_move(&mut self, action: &Move){
        self.play_safe(&action);
    }

    fn reward(&self) -> f32 {
        let win_indicator = if self.turn() == Color::White { 1. } else { -1. };
        let outcome = self.outcome();
        match outcome {
            Some(Outcome::Decisive { winner: Color::Black }) =>  -1.0 * win_indicator,
            Some(Outcome::Decisive { winner: Color::White }) => 1.0 * win_indicator,
            Some(Outcome::Draw) => 0.0,
            None => 0.0
        }
    }

    fn set_rng_seed(&mut self, seed: u32){

    }
}

impl mcts::GameAction for Move{}

pub fn main() {
    let starting_position = Chess::default();
    let mut game = starting_position.clone();
    let move_history = play_game(&mut game, 4, true, 100.);
    let pgn = pgn::to_pgn(starting_position, move_history);
    println!("{}", pgn);
}

pub fn play_game(game: &mut Chess, ensemble_size: usize,
                 verbose: bool, time_per_move_ms: f32) -> Vec<Move>{
    let mut mcts: MCTS<Chess, Move> = MCTS::new(&game, ensemble_size);

    let mut move_history: Vec<Move> = Vec::new();

    loop {
        let t0 = Instant::now();
        mcts.search_time(time_per_move_ms, 1.);

        if verbose {
            println!("{:?}", mcts.tree_statistics());
        }

        let action = mcts.best_action();
        match action {
            Some(action) => {
                game.make_move(&action);
                mcts.advance_game(&game);
                println!("{:?}\n{:?}", action, game.board());
                move_history.push(action);
            },
            None => break
        }
        let time_spend = t0.elapsed().as_millis();
        println!("move time: {}s", time_spend);
    }
    println!("{:?}", game);
    move_history
}

// pub fn search(game: &mut Game, pos: &Chess, depth: u16) {
//     let mut moves = MoveList::new();
//     pos.legal_moves(&mut moves);
//     // println!("possible moves: {:?}", moves.len());
//
//     let positions: Vec<_> = moves
//         .drain(..)
//         .map(|m| {
//             let mut child_pos = pos.clone();
//             child_pos.play_unchecked(&m);
//             return EvaluatedMove {
//                 evaluated_move: m,
//                 position: child_pos.clone(),
//                 outcome: child_pos.outcome().clone(),
//             };
//         }).collect();
//     // println!("positions {:?}", &positions);
//
//     let decisive_move: Option<&EvaluatedMove> = positions.iter().find(|em| match em.outcome {
//         Some(o) => match o {
//             Outcome::Decisive { winner: _ } => true,
//             _ => false,
//         },
//         _ => false,
//     });
//
//     match decisive_move {
//         Some(em) => {
//             println!("decisive move {:?}", &em);
//             game.moves.push(em.evaluated_move.clone());
//             game.outcome = em.outcome.clone();
//         }
//         None => {
//             let non_stalemate_moves: Vec<_> = positions.iter().filter(|em| match em.outcome {
//                 None => true,
//                 _ => false
//             }).collect();
//             match non_stalemate_moves.is_empty() {
//                 true => {
//                     let first_stalemate: &EvaluatedMove = positions.first().unwrap();
//                     assert!(matches!(first_stalemate.outcome.unwrap(), Outcome::Draw));
//                     game.moves.push(first_stalemate.evaluated_move.clone());
//                     game.outcome = first_stalemate.outcome.clone();
//                 },
//                 false  => {
//
//                     let mut rng = thread_rng();
//                     let rand_sample = sample(&mut rng, &non_stalemate_moves, 1);
//                     println!("sample {:?}", rand_sample);
//                     let random_position = rand_sample.first().unwrap();
//                     println!("position chosen");
//                     game.moves.push(random_position.evaluated_move.clone());
//                     if depth > 0 {
//                         search(game, &random_position.position, depth - 1);
//                     }
//                 }
//             }
//         }
//     }
// }

