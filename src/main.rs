extern crate shakmaty;
extern crate deadbeef;

use shakmaty::*;
use deadbeef::play;
use deadbeef::pgn;


pub fn main() {
    let starting_position = Chess::default();
    let move_history = play::play_game(&starting_position, 12, 200.0, 0.01);

    let pgn = pgn::to_pgn(&starting_position, &move_history);
    println!("{}", pgn);
}

