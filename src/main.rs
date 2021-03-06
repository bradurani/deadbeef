extern crate deadbeef;
extern crate log;

use deadbeef::engine::*;
use deadbeef::logger;
use deadbeef::settings::Settings;
use deadbeef::xboard::XBoard;
use log::*;

//TODO remove Copy from Move in Shakmaty

pub fn main() {
    logger::init();

    let settings: Settings = Default::default();

    let mut engine: Engine = Engine::new(settings);
    let mut xboard: XBoard = Default::default();
    xboard.start(&mut engine);
    warn!("exiting!");
}
