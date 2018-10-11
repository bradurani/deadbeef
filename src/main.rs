extern crate deadbeef;
extern crate shakmaty;

use deadbeef::pgn;
use deadbeef::play;
use deadbeef::settings::*;
use shakmaty::*;

//TODO remove Copy from Move in Shakmaty
pub fn main() {
    let settings = Settings {
        starting_position: Chess::default(),
        starting_move_num: 1.0,
        time_per_move_ms: -1.0,
        n_samples: 1000,
        ensemble_size: 12,
        c: 0.25,
        starting_seed: 1,
    };

    let move_history = play::play_game(&settings);

    let pgn = pgn::to_pgn(&settings.starting_position, &move_history);
    println!("{}", pgn);
}
