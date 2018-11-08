extern crate deadbeef;

use deadbeef::engine::*;
use deadbeef::xboard::XBoard;

//TODO remove Copy from Move in Shakmaty

pub fn main() {
    let mut engine: Engine = Default::default();
    let mut xboard: XBoard = Default::default();
    xboard.start(&mut engine);
}
