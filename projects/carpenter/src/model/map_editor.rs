use std::path::{PathBuf};

use slog::{Logger};
use stbus::{Bus, BusReader};

use model::autosave::{Autosave};

pub struct MapEditor {
    event_bus: Bus<MapEditorEvent>,
    autosave: Option<Autosave>,
}

impl MapEditor {
    pub fn new() -> Self {
        MapEditor {
            event_bus: Bus::new(),
            autosave: None,
        }
    }

    pub fn subscribe(&mut self) -> BusReader<MapEditorEvent> {
        self.event_bus.add_rx()
    }

    pub fn new_brush(&mut self) {
        self.event_bus.broadcast(&MapEditorEvent::NewBrush);
    }

    pub fn set_save_target(&mut self, target: PathBuf) {
        self.autosave = Some(Autosave::new(target));
    }

    pub fn update(&mut self, delta: f32, log: &Logger) {
        if let Some(ref mut autosave) = self.autosave {
            autosave.update(delta, log);
        }
    }
}

#[derive(Clone, Debug)]
pub enum MapEditorEvent {
    NewBrush,
}
