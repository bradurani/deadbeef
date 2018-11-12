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

    pub fn recalculate_from_now(&self) -> TimeRemaining {
        let now = Instant::now();
        let time_spent = now - self.start;
        TimeRemaining {
            start: now,
            remaining: self
                .remaining
                .checked_sub(time_spent)
                .unwrap_or(Duration::from_millis(0)),
        }
    }
}

impl ToString for TimeRemaining {
    fn to_string(&self) -> String {
        format!("{:?}", self.remaining)
    }
}
