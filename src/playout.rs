use game::*;
use stats::RunStats;
use rand::Rng;
use shakmaty::{Chess, Position};
use utils::choose_random;

const MAX_PLAYOUT_MOVES: u32 = 4000;

/// Perform a random playout.
///
/// Start with an initial game state and perform random actions from
/// until a game-state is reached that does not have any `allowed_actions`.
pub fn playout<R: Rng>(rng: &mut R, initial: &Chess, thread_run_stats: &mut RunStats) -> Chess {
    let mut game = initial.clone();

    let mut potential_moves = game.allowed_actions();

    let mut num_moves = 0;
    while potential_moves.len() > 0 && !game.is_insufficient_material() {
        num_moves += 1;
        if num_moves >= MAX_PLAYOUT_MOVES {
            eprintln!("REACHED MAX PLAYOUT LENGTH");
            thread_run_stats.maxouts += 1;
            break;
        }
        {
            let action = choose_random(rng, &potential_moves);
            game.make_move(&action);
            thread_run_stats.playout_moves += 1;
        }
        potential_moves = game.allowed_actions();
    }
    thread_run_stats.playouts += 1;
    game
}
