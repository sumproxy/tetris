use ::std::time::{Duration, Instant};

pub struct Timer {
    accumulator: Instant,
    threshold: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            accumulator: Instant::now(),
            threshold: Duration::from_millis(350),
        }
    }

    pub fn is_up(&mut self) -> bool {
        if self.accumulator.elapsed() > self.threshold {
            self.accumulator = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn lower_threshold(&mut self) {
        if self.threshold - Duration::from_millis(5) >= Duration::from_millis(100) {
            self.threshold -= Duration::from_millis(5);
        }
    }
}