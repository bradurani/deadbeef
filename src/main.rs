extern crate deadbeef;
extern crate shakmaty;

use deadbeef::pgn;
use deadbeef::play;
use deadbeef::settings::*;
use std::env::*;

//TODO remove Copy from Move in Shakmaty
pub fn main() {
    let args: Vec<String> = args().collect();
    let settings = Settings::game_default();
    let move_history = if args[1] == "2" {
        play::play_2_player_game(&settings)
    } else {
        play::play_game(&settings)
    };
    let pgn = pgn::to_pgn(&settings.starting_position, &move_history);
    println!("{}", pgn);
}
