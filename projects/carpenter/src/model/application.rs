use bus::{Bus, BusReader};

pub struct Application {
    event_bus: Bus<ApplicationEvent>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            event_bus: Bus::new(100),
        }
    }

    pub fn subscribe(&mut self) -> BusReader<ApplicationEvent> {
        self.event_bus.add_rx()
    }

    pub fn new_brush(&mut self) {
        self.event_bus.try_broadcast(ApplicationEvent::NewBrush).unwrap();
    }
}

#[derive(Clone, Debug)]
pub enum ApplicationEvent {
    NewBrush,
}
