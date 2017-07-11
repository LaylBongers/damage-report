pub struct AverageDelta {
    last_average: f32,

    accum_delta: f32,
    accum_frames: i32,
    accum_second: f32,
}

/// A counter that calculates an average delta every second. To be used by FPS counters that need
/// to have a stable easier to read value.
impl AverageDelta {
    pub fn new() -> AverageDelta {
        AverageDelta {
            last_average: 1.0,
            accum_delta: 0.0,
            accum_frames: 0,
            accum_second: 0.0,
        }
    }

    pub fn accumulate(&mut self, delta: f32) {
        self.accum_delta += delta;
        self.accum_frames += 1;
        self.accum_second += delta;
        if self.accum_second >= 1.0 {
            self.last_average = self.accum_delta / self.accum_frames as f32;
            self.accum_delta = 0.0;
            self.accum_frames = 0;
            self.accum_second = 0.0;
        }
    }

    pub fn get(&self) -> f32 {
        self.last_average
    }
}
