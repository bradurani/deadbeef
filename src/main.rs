extern crate deadbeef;
extern crate log;

use deadbeef::engine::*;
use deadbeef::logger;
use deadbeef::logger::*;
use deadbeef::xboard::XBoard;
use deadbeef::Settings::*;
#[macro_use]
use log::*;

//TODO remove Copy from Move in Shakmaty

pub fn main() {
    logger::init();

    let settings: Settings = Default::default();
    info!("{}", settings);

    let mut engine: Engine = Engine::new(settings);
    let mut xboard: XBoard = Default::default();
    xboard.start(&mut engine);
}
