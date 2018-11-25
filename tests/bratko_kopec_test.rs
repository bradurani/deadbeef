#[macro_use]
extern crate log;
extern crate deadbeef;

use helpers::*;
mod helpers;

#[test]
#[ignore]
fn bratko_kopec() {
    let times = vec![5000, 5000, 5000];
    let score = run_challenge_suite("data/bratko_kopec_test.epd", &times);
    info!("bratko-kopec test score {} at {:?}", score, times);
}
