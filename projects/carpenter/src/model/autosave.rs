use slog::{Logger};
use std::path::{PathBuf};

pub struct Autosave {
    target: PathBuf,
    interval: f32,
    accumulator: f32,
}

impl Autosave {
    pub fn new(target: PathBuf) -> Self {
        Autosave {
            target,
            interval: 60.0,
            accumulator: 60.0, // Makes sure we start with a save
        }
    }

    pub fn update(&mut self, delta: f32, log: &Logger) {
        self.accumulator += delta;
        if self.accumulator >= self.interval {
            info!(log, "TODO: Saving to {}!", self.target.display());
            self.accumulator = 0.0;
        }
    }
}
