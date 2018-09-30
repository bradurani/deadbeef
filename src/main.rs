extern crate rand;
extern crate shakmaty;

use rand::{sample, thread_rng};
use shakmaty::{Chess, Move, MoveList, Outcome, Position, Role, Square};
use std::fmt::Debug;

pub enum GamePath {
    Empty,
    Cons(Move, Box<GamePath>),
}

pub struct Game {
    moves: GamePath,
    outcome: Option<Outcome>,
}

fn main() {
    let pos = Chess::default();
    let path = search(&pos, 3);
    println!("{:?}", pos);
}

pub fn search<P: Position + Clone + Debug>(pos: &P, depth: u8) -> Game {
    let mut moves = MoveList::new();
    pos.legal_moves(&mut moves);
    let mut positions = moves.drain(..).map(|m| {
        let mut child = pos.clone();
        child.play_unchecked(&m);
        (m, child.clone(), child.outcome())
    });
    let decisive_move = positions.find(|(m, c, o)| match o {
        Decisive => true,
    });
    match decisive_move {
        Some(t) => Game {
            moves: GamePath::Cons(t.0, Box::new(GamePath::Empty)),
            outcome: t.2,
        },
        None => {
            let mut rng = thread_rng();
            match sample(&mut rng, positions, 1).first() {
                Some(position) => {
                    let mut game = search(&position.1, depth - 1);
                    game.moves = GamePath::Cons(position.0.clone(), Box::new(game.moves));
                    return game;
                }
                None => {
                    panic!("invalid position list");
                }
            }
        }
    }
}
