use std::time::{Duration, Instant};

pub struct FixedTimer {
    interval: Duration,
    next_time: Instant,
}

impl FixedTimer {
    pub fn new(interval: Duration) -> Self {
        Self { interval, next_time: Instant::now() + interval }
    }

    pub fn try_tick(&mut self, now: Instant) -> bool {
        if now >= self.next_time {
            self.next_time += self.interval;
            true
        } else {
            false
        }
    }
}