extern crate deadbeef;
extern crate shakmaty;

use deadbeef::game::Game;
use helpers::*;
use shakmaty::fen::*;

mod helpers;

#[test]
fn opening_move() {
    assert_move(STARTING_POSITION, "e2e4");
}
