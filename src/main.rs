extern crate deadbeef;
extern crate shakmaty;

use deadbeef::pgn;
use deadbeef::play;
use deadbeef::settings::*;

//TODO remove Copy from Move in Shakmaty
pub fn main() {
    let settings = Settings::game_default();
    let move_history = play::play_game(&settings);
    let pgn = pgn::to_pgn(&settings.starting_position, &move_history);
    println!("{}", pgn);
}
