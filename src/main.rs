extern crate deadbeef;
extern crate shakmaty;

use deadbeef::pgn;
use deadbeef::play;
use shakmaty::*;

//TODO remove Copy from Move in Shakmaty
pub fn main() {
    let starting_position = Chess::default();
    let move_history = play::play_game(&starting_position, 12, 1000.0, 0.25, 1, -1);

    let pgn = pgn::to_pgn(&starting_position, &move_history);
    println!("{}", pgn);
}
