use eval::*;
use game::*;
use settings::Settings;
use shakmaty::{Chess, Position, Setup};
use stats::RunStats;

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
                Some(o) => o.reward(),
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
            value = (-negamax(
                child_position,
                depth - 1,
                -beta,
                -alpha,
                -coefficient,
                stats,
            ))
            .max(value);
            alpha = alpha.max(value);
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
    ) * starting_coefficient
}
