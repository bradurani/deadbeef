extern crate deadbeef;
extern crate shakmaty;

use deadbeef::pgn;
use deadbeef::play;
use deadbeef::settings::*;
use std::env::*;

//TODO remove Copy from Move in Shakmaty
pub fn main() {
    let args: Vec<String> = args().collect();
    let settings = Settings::parse_args(&args);
    let move_history = match args.get(1).as_ref().map(|s| &s[..]) {
        Some("2") => play::play_2_player_game(&settings),
        Some("m") => play::play_move(&settings),
        None => play::play_game(&settings),
        _ => panic!("unknown cmd line arg"),
    };
    let pgn = pgn::to_pgn(&settings.starting_position, &move_history);
    println!("{}", pgn);
}
