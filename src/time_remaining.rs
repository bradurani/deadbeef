use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct TimeRemaining {
    start: Instant,
    remaining: Duration,
}

impl TimeRemaining {
    pub fn start(remaining: Duration) -> TimeRemaining {
        TimeRemaining {
            start: Instant::now(),
            remaining: remaining,
        }
    }
}
