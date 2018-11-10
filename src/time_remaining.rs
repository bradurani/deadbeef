use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
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

impl ToString for TimeRemaining {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

