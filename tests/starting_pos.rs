extern crate deadbeef;
extern crate shakmaty;

use helpers::*;
use shakmaty::fen::*;

mod helpers;

#[test]
fn opening_move() {
    assert_move(Fen::STARTING_POSITION, "e4e5");
}
