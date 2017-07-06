use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface};
use vulkano_win::{self, VkSurfaceBuild, Window as VulkanWinWindow};
use winit::{EventsLoop, WindowBuilder, Event, WindowEvent};

use calcium_rendering_vulkano::{VulkanoTargetSystem};

pub struct WinitTargetSystem {
    data: Option<WinitData>,
}

impl WinitTargetSystem {
    pub fn new() -> Self {
        WinitTargetSystem {
            data: None,
        }
    }

    pub fn handle_events(&mut self) -> bool {
        let data = self.data.as_mut().unwrap();
        let mut should_continue = true;

        data.events_loop.poll_events(|event| {
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

impl VulkanoTargetSystem for WinitTargetSystem {
    fn required_extensions(&self) -> InstanceExtensions {
        vulkano_win::required_extensions()
    }

    fn create_surface(&mut self, instance: Arc<Instance>, size: Vector2<u32>) -> Arc<Surface> {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_dimensions(size.x, size.y)
            .with_title(format!("Trenches"))
            .build_vk_surface(&events_loop, instance)
            .unwrap();
        self.data = Some(WinitData {
            window,
            events_loop,
        });
        self.data.as_ref().unwrap().window.surface().clone()
    }
}

pub struct WinitData {
    window: VulkanWinWindow,
    events_loop: EventsLoop,
}
