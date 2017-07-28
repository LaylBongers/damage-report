use std::path::{PathBuf};

use slog::{Logger};

use autosave::{Autosave};
use map::{Map, Brush};
use input::{InputModel};
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
        self.map.brushes.push(Brush::cube());
        self.event_bus.broadcast(&MapEditorEvent::NewBrush(self.map.brushes.len() - 1));
    }

    pub fn brush(&self, index: usize) -> &Brush {
        &self.map.brushes[index]
    }

    pub fn set_save_target(&mut self, target: PathBuf) {
        self.autosave = Some(Autosave::new(target));
    }

    pub fn update(&mut self, delta: f32, input: &InputModel, log: &Logger) -> Result<(), Error> {
        // Check if we got a select click
        if input.primary_action.pressed {
            info!(log, "Select!");
        }

        // Check if saving has been enabled, and if so, update the autosave model
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
    NewBrush(usize),
}
