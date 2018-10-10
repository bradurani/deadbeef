use mcts::Game;
use rand::Rng;
use shakmaty::{Chess, Position};
use utils::choose_random;

const MAX_PLAYOUT_MOVES: u32 = 4000;

/// Perform a random playout.
///
/// Start with an initial game state and perform random actions from
/// until a game-state is reached that does not have any `allowed_actions`.
pub fn playout<R: Rng>(rng: &mut R, initial: &Chess) -> Chess {
    let mut game = initial.clone();

    let mut potential_moves = game.allowed_actions();

    let mut num_moves = 0;
    while potential_moves.len() > 0 && !game.is_insufficient_material() {
        num_moves += 1;
        if num_moves >= MAX_PLAYOUT_MOVES {
            eprintln!("REACHED MAX PLAYOUT LENGTH");
            break;
        }
        {
            let action = choose_random(rng, &potential_moves);
            game.make_move(&action);
        }
        potential_moves = game.allowed_actions();
    }
    game
}
