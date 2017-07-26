use std::path::{PathBuf};
use std::fs::{File, create_dir_all};

use slog::{Logger};

use map::{Map};
use {Error};

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

    pub fn update(&mut self, delta: f32, map: &Map, log: &Logger) -> Result<(), Error> {
        self.accumulator += delta;
        if self.accumulator >= self.interval {
            self.force_save(map, log)?;
            self.accumulator = 0.0;
        }

        Ok(())
    }

    pub fn force_save(&self, map: &Map, log: &Logger) -> Result<(), Error> {
        info!(log, "Saving map to \"{}\"", self.target.display());

        // TODO: Backup previous save before overwriting

        // First make sure the directory at this path exists
        let mut dir = self.target.clone();
        dir.pop();
        create_dir_all(dir)?;

        // Then we can actually serialize and save the data to there
        let mut file = File::create(&self.target)?;
        ::serde_json::to_writer(&mut file, map)?;

        Ok(())
    }
}
