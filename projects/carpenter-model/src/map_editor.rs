use std::path::{PathBuf};

use slog::{Logger};

use autosave::{Autosave};
use map::{Map, Brush};
use {Bus, BusReader, Error};

pub struct MapEditor {
    event_bus: Bus<MapEditorEvent>,
    autosave: Option<Autosave>,
    map: Map,
}

impl MapEditor {
    pub fn new() -> Self {
        MapEditor {
            event_bus: Bus::new(),
            autosave: None,
            map: Map::new(),
        }
    }

    pub fn subscribe(&mut self) -> BusReader<MapEditorEvent> {
        self.event_bus.add_rx()
    }

    pub fn new_brush(&mut self) {
        self.map.brushes.push(Brush::new());
        self.event_bus.broadcast(&MapEditorEvent::NewBrush);
    }

    pub fn set_save_target(&mut self, target: PathBuf) {
        self.autosave = Some(Autosave::new(target));
    }

    pub fn update(&mut self, delta: f32, log: &Logger) -> Result<(), Error> {
        if let Some(ref mut autosave) = self.autosave {
            autosave.update(delta, &self.map, log)?;
        }

        Ok(())
    }

    pub fn force_save(&mut self, log: &Logger) -> Result<(), Error> {
        if let Some(ref mut autosave) = self.autosave {
            autosave.force_save(&self.map, log)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum MapEditorEvent {
    NewBrush,
}
