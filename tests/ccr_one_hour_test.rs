#[macro_use]
extern crate log;
extern crate deadbeef;

use helpers::*;
mod helpers;

#[test]
#[ignore]
fn ccr_one_hour() {
    let times = vec![5000, 5000, 5000];
    let score = run_challenge_suite("data/ccr_one_hour_test.epd", &times);
    info!("ccr one hour test score {} at {:?}", score, times);
}
