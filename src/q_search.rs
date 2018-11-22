use eval::*;
use game::*;
use shakmaty::{Chess, MoveList, Position, Setup};
use stats::RunStats;
use std::cmp::max;

pub fn q_search(
    position: Chess,
    depth: usize,
    mut alpha: Reward,
    beta: Reward,
    coefficient: Reward,
    stats: &mut RunStats,
) -> Reward {
    stats.record_q_depth(depth);
    if position.is_game_over() {
        return coefficient * position.outcome().unwrap().reward();
    };
    let mut value = coefficient * position.board().value(); // is this a NULL move?
    stats.evals += 1;
    if value > alpha {
        alpha = value
    }
    let mut capture_moves = MoveList::new();
    position.capture_moves(&mut capture_moves);
    for child_move in capture_moves {
        // TODO do we get capture moves if in check
        // TODO should add promotions and other big moves
        let mut child_position = position.clone();
        child_position.play_unchecked(&child_move);
        // info_emojified(&child_position.board());
        value = max(
            -q_search(
                child_position,
                depth + 1,
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
            break;
        }
    }
    value
}

// fn print_value(child_move: Move, value: Reward, depth: isize) {
//     let spaces = (0..(5 * (20 - (depth + 1))))
//         .map(|_| " ")
//         .collect::<String>();
//     info!("{} {} {}", spaces, child_move, value);
// }
