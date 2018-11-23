use game::*;
use q_search::*;
use settings::Settings;
use shakmaty::{Chess, Position, Setup};
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
        settings: &Settings,
    ) -> Reward {
        stats.record_playout_depth(depth);
        if position.is_game_over() {
            return coefficient * position.outcome().unwrap().reward();
        };
        if depth == 0 {
            stats.playout_leaves += 1;
            return q_search(position, 0, alpha, beta, coefficient, stats, settings);
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
                    settings,
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
    negamax(
        starting_position,
        settings.playout_depth,
        MIN_REWARD,
        MAX_REWARD,
        starting_coefficient,
        stats,
        settings,
    ) * starting_coefficient
}

// fn print_value(child_move: Move, value: Reward, depth: isize) {
//     let spaces = (0..(5 * (20 - (depth + 1))))
//         .map(|_| " ")
//         .collect::<String>();
//     info!("{} {} {}", spaces, child_move, value);
// }
