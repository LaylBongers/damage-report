use std::time::{Instant};

pub struct LoopTimer {
    last: Instant,
}

impl LoopTimer {
    pub fn start() -> Self {
        LoopTimer {
            last: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> f32 {
        // First take a time measurement
        let now = Instant::now();
        let duration = now.duration_since(self.last);
        self.last = now;

        // If it took over one second, cap it at a second
        if duration.as_secs() != 0 {
            return 1.0
        }

        // Get the nanoseconds and convert it to a seconds float
        let nanos = duration.subsec_nanos();
        let delta = nanos as f32 / 1_000_000_000.0;

        // Make sure the final delta isn't negative or absolute zero
        f32::max(delta, 0.000_000_1)
    }
}

pub fn delta_to_fps(delta: f32) -> f32 {
    1.0 / delta
}
