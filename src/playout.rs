use eval::*;
use game::*;
use settings::Settings;
use shakmaty::{Chess, Position, Setup};
use stats::RunStats;
use std::i16;

pub fn playout(starting_position: Chess, stats: &mut RunStats, settings: &Settings) -> Reward {
    fn negamax(
        position: Chess,
        depth: usize,
        mut alpha: Reward,
        beta: Reward,
        coefficient: isize,
        stats: &mut RunStats,
    ) -> Reward {
        if depth == 0 || position.has_outcome() {
            return match position.outcome() {
                Some(o) => o.reward(),
                None => {
                    stats.evals += 1;
                    position.board().value()
                }
            };
        }
        // TODO try the chess crate here
        let mut value = i16::MIN;
        for child_move in position.legals() {
            let mut child_position = position.clone(); //TODO can we apply and undo?
            child_position.play_unchecked(&child_move);
            value = -negamax(
                child_position,
                depth - 1,
                -beta,
                -alpha,
                -coefficient,
                stats,
            )
            .max(value);
            alpha = alpha.max(value);
            if alpha >= beta {
                break; // the possibilites from the position are better than from other siblings, so our opponnent won't give us this position. We can stop evaluatin
            }
        }
        value
    }

    let starting_coefficient = starting_position.turn().coefficient() as isize;
    negamax(
        starting_position,
        settings.playout_depth,
        i16::MIN + 1, // for some reason, min is -32768 but max is 32767
        i16::MAX,
        starting_coefficient,
        stats,
    )
}
