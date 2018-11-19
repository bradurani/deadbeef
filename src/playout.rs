use eval::*;
use game::*;
use settings::Settings;
use shakmaty::{Chess, Move, Position, Setup};
use stats::RunStats;
use std::cmp::max;

pub fn playout(starting_position: Chess, stats: &mut RunStats, settings: &Settings) -> Reward {
    fn negamax(
        position: Chess,
        depth: usize,
        mut alpha: Reward,
        beta: Reward,
        coefficient: Reward,
        stats: &mut RunStats,
    ) -> Reward {
        if depth == 0 || position.has_outcome() {
            return match position.outcome() {
                Some(o) => coefficient * o.reward(),
                None => {
                    stats.evals += 1;
                    coefficient * position.board().value()
                }
            };
        }
        // TODO try the chess crate here
        let mut value = MIN_REWARD;
        for child_move in position.legals() {
            let mut child_position = position.clone(); //TODO can we apply and undo?
            child_position.play_unchecked(&child_move);
            value = max(
                -negamax(
                    child_position,
                    depth - 1,
                    -beta,
                    -alpha,
                    -coefficient,
                    stats,
                ),
                value,
            );
            // print_value(child_move, value, depth);
            alpha = max(alpha, value);
            if alpha >= beta {
                break; // the possibilites from the position are better than from other siblings, so our opponnent won't give us this position. We can stop evaluatin
            }
        }
        value
    }

    let starting_coefficient = starting_position.turn().coefficient();
    println!("starting co {}", starting_coefficient);
    negamax(
        starting_position,
        settings.playout_depth,
        MIN_REWARD,
        MAX_REWARD,
        starting_coefficient,
        stats,
    ) * starting_coefficient
}

// fn print_value(child_move: Move, value: Reward, depth: usize) {
//     // if depth > 2 {
//     let spaces = (0..(10 * (10 - depth))).map(|_| " ").collect::<String>();
//     info!("{} {} {}", spaces, child_move, value);
//     // }
// }
