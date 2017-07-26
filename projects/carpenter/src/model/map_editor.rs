use std::path::{PathBuf};
use stbus::{Bus, BusReader};

pub struct MapEditorModel {
    save_target: Option<PathBuf>,
    event_bus: Bus<MapEditorEvent>,
}

impl MapEditorModel {
    pub fn new() -> Self {
        MapEditorModel {
            save_target: None,
            event_bus: Bus::new(),
        }
    }

    pub fn subscribe(&mut self) -> BusReader<MapEditorEvent> {
        self.event_bus.add_rx()
    }

    pub fn new_brush(&mut self) {
        self.event_bus.broadcast(&MapEditorEvent::NewBrush);
    }

    pub fn set_save_target(&mut self, target: PathBuf) {
        println!("Target: {}", target.display());
        self.save_target = Some(target);
    }
}

#[derive(Clone, Debug)]
pub enum MapEditorEvent {
    NewBrush,
}
