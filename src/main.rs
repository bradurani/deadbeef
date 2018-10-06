#![feature(duration_as_u128)]

extern crate rand;
extern crate shakmaty;
extern crate matches;
extern crate itertools;

use shakmaty::*;
use mcts::{Game, MCTS, TreeNode};
use std::time::{Instant};
use tree_merge::merge_trees;

pub mod pgn;
pub mod mcts;
pub mod utils;
pub mod tree_merge;

impl mcts::Game for Chess {
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
        self.play_unchecked(&action);
        // self.play_safe(&action)
        // TODO add safe option for testing
    }

    fn reward(&self) -> f32 {
        let outcome = self.outcome();
        match outcome {
            Some(Outcome::Decisive { winner: Color::Black }) =>  -1.0,
            Some(Outcome::Decisive { winner: Color::White }) => 1.0,
            Some(Outcome::Draw) => 0.0,
            None => 0.0
        }
    }

    fn set_rng_seed(&mut self, _seed: u32){

    }
}

pub fn main() {
    let starting_position = Chess::default();
    let mut game = starting_position.clone();
    let move_history = play_game(&mut game, 4, true, 2000.0, 0.01);
    let pgn = pgn::to_pgn(starting_position, move_history);
    println!("{}", pgn);
}

pub fn play_game(game: &mut Chess, ensemble_size: usize,
                 verbose: bool, time_per_move_ms: f32, exploration_param: f32) -> Vec<Move>{

    let mut move_history: Vec<Move> = Vec::new();
    let mut move_num = 0.5;
    let mut mcts: MCTS = MCTS::new();
    let mut merged_root = TreeNode::new_root(game, move_num);

    loop {
        move_num += 0.5;
        println!("\nMove: {}", move_num);
        let t0 = Instant::now();
        {
        println!("Starting with {:?}", mcts.tree_statistics(&vec![merged_root.clone()]));
        println!("{}", merged_root);
        }
        let roots = mcts.search_time(merged_root, game, ensemble_size,
                                     time_per_move_ms, exploration_param, move_num);

        if verbose {
            println!("{:?}", mcts.tree_statistics(&roots));
        }

        let best_children = mcts.best_children(roots);
        match best_children {
            None => break,
            Some(children) => {
                merged_root = merge_trees(children);
                let action = merged_root.action.unwrap();
                game.make_move(&action);
                println!("Moving: {}\n{:?}", &action, game.board());
                move_history.push(action);
            },
        }
        let time_spend = t0.elapsed().as_millis();
        println!("move time: {}ms", time_spend);
    }
    move_history
}
