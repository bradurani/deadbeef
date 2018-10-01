extern crate rand;
extern crate shakmaty;
#[macro_use]
extern crate matches;

use rand::{sample, thread_rng};
use shakmaty::{Chess, Move, MoveList, Outcome, Position, Role, Square};

pub mod pgn;

#[derive(Debug, Default)]
pub struct Game {
    moves: Vec<Move>,
    outcome: Option<Outcome>,
}

#[derive(Debug)]
pub struct EvaluatedMove {
    evaluated_move: Move,
    position: Chess,
    outcome: Option<Outcome>,
}

pub fn main() {
    let pos = Chess::default();
    let mut game = Game::default();
    search(&mut game, &pos, 300);
    let pgn = pgn::to_pgn(game.moves, game.outcome);
    println!("{}", pgn);
}

pub fn search(game: &mut Game, pos: &Chess, depth: u16) {
    let mut moves = MoveList::new();
    pos.legal_moves(&mut moves);
    // println!("possible moves: {:?}", moves.len());

    let positions: Vec<_> = moves
        .drain(..)
        .map(|m| {
            let mut child_pos = pos.clone();
            child_pos.play_unchecked(&m);
            return EvaluatedMove {
                evaluated_move: m,
                position: child_pos.clone(),
                outcome: child_pos.outcome().clone(),
            };
        }).collect();
    // println!("positions {:?}", &positions);

    let decisive_move: Option<&EvaluatedMove> = positions.iter().find(|em| match em.outcome {
        Some(o) => match o {
            Outcome::Decisive { winner: _ } => true,
            _ => false,
        },
        _ => false,
    });

    match decisive_move {
        Some(em) => {
            println!("decisive move {:?}", &em);
            game.moves.push(em.evaluated_move.clone());
            game.outcome = em.outcome.clone();
        }
        None => {
            let non_stalemate_moves: Vec<_> = positions.iter().filter(|em| match em.outcome {
                None => true,
                _ => false
            }).collect();
            match non_stalemate_moves.is_empty() {
                true => {
                    let first_stalemate: &EvaluatedMove = positions.first().unwrap();
                    assert!(matches!(first_stalemate.outcome.unwrap(), Outcome::Draw));
                    game.moves.push(first_stalemate.evaluated_move.clone());
                    game.outcome = first_stalemate.outcome.clone();
                },
                false  => {

                    let mut rng = thread_rng();
                    let rand_sample = sample(&mut rng, &non_stalemate_moves, 1);
                    println!("sample {:?}", rand_sample);
                    let random_position = rand_sample.first().unwrap();
                    println!("position chosen");
                    game.moves.push(random_position.evaluated_move.clone());
                    if depth > 0 {
                        search(game, &random_position.position, depth - 1);
                    }
                }
            }
        }
    }
}
