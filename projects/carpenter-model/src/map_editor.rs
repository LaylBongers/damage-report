use std::path::{PathBuf};

use slog::{Logger};
use cgmath::{Point3};

use autosave::{Autosave};
use map::{Map, Brush};
use {Bus, BusReader, Error};

pub struct MapEditor {
    event_bus: Bus<MapEditorEvent>,
    autosave: Option<Autosave>,
    map: Map,
    selected_brushes: Vec<usize>,
}

impl MapEditor {
    pub fn new() -> Self {
        MapEditor {
            event_bus: Bus::new(),
            autosave: None,
            map: Map::new(),
            selected_brushes: Vec::new(),
        }
    }

    pub fn set_save_target(&mut self, target: PathBuf) {
        self.autosave = Some(Autosave::new(target));
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn subscribe(&mut self) -> BusReader<MapEditorEvent> {
        self.event_bus.add_rx()
    }

    pub fn new_brush(&mut self, position: Point3<f32>) {
        self.map.brushes.push(Brush::cube(position));
        self.event_bus.broadcast(&MapEditorEvent::NewBrush(self.map.brushes.len() - 1));
    }

    pub fn deselect_all(&mut self) {
        self.selected_brushes.clear()
    }

    pub fn select(&mut self, index: usize) {
        if !self.selected_brushes.iter().any(|v| *v == index) {
            self.selected_brushes.push(index);
        }
    }

    pub fn update(&mut self, delta: f32, log: &Logger) -> Result<(), Error> {
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
