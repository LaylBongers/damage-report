use bus::{Bus, BusReader};

pub struct MapEditor {
    event_bus: Bus<MapEditorEvent>,
}

impl MapEditor {
    pub fn new() -> Self {
        MapEditor {
            event_bus: Bus::new(100),
        }
    }

    pub fn subscribe(&mut self) -> BusReader<MapEditorEvent> {
        self.event_bus.add_rx()
    }

    pub fn new_brush(&mut self) {
        self.event_bus.try_broadcast(MapEditorEvent::NewBrush).unwrap();
    }
}

#[derive(Clone, Debug)]
pub enum MapEditorEvent {
    NewBrush,
}
