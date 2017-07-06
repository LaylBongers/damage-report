extern crate cgmath;
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;
extern crate calcium_window;

use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::swapchain::{Surface};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano_win::{VkSurfaceBuild, Window as VulkanoWinWindow};
use winit::{EventsLoop, WindowBuilder, Event, WindowEvent};
use calcium_window::{Window};

pub fn required_extensions() -> InstanceExtensions {
    vulkano_win::required_extensions()
}

pub struct WinitWindow {
    pub window: VulkanoWinWindow,
    pub events_loop: EventsLoop,
    pub surface: Arc<Surface>,
}

impl WinitWindow {
    pub fn new_vulkano(instance: Arc<Instance>, title: &str, size: Vector2<u32>) -> Self {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_dimensions(size.x, size.y)
            .with_title(title)
            .build_vk_surface(&events_loop, instance)
            .unwrap();

        let surface = window.surface().clone();

        WinitWindow {
            window,
            events_loop,
            surface,
        }
    }
}

impl Window for WinitWindow {
    fn handle_events(&mut self) -> bool {
        let mut should_continue = true;

        self.events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: ev, .. } => {
                    match ev {
                        WindowEvent::Closed => should_continue = false,
                        _ => {},
                    }
                },
                _ => {},
            }
        });

        should_continue
    }
}
