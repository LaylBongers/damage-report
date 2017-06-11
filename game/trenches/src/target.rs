use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface};
use vulkano_win::{self, VkSurfaceBuild, Window as VulkanWinWindow};
use winit::{EventsLoop, WindowBuilder, Event, WindowEvent};

use calcium_rendering_vulkano::{VulkanoTargetSystem};

use input::{InputState, FrameInput};

pub struct WinitTargetSystem {
    data: Option<WinitData>,
}

impl WinitTargetSystem {
    pub fn new() -> Self {
        WinitTargetSystem {
            data: None,
        }
    }

    pub fn handle_events(
        &mut self,
        input_state: &mut InputState, frame_input: &mut FrameInput
    ) -> bool {
        let data = self.data.as_mut().unwrap();
        let mut should_continue = true;

        data.events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: ev, .. } => {
                    match ev {
                        WindowEvent::Closed => should_continue = false,
                        WindowEvent::KeyboardInput(key_state, _, Some(key_code), _) =>
                            input_state.handle_key(key_state, key_code),
                        WindowEvent::MouseMoved(x, y) => {
                            let center = (data.size/2).cast();

                            // Check how far away from the center we are and use that to calculate input
                            let difference: Vector2<i32> = Vector2::new(x, y) - center;
                            frame_input.pitch += difference.y as f32 * -0.0005;
                            frame_input.yaw += difference.x as f32 * -0.0005;

                            // Re-center the mouse so it stays in the middle of the screen
                            data.window.window()
                                .set_cursor_position(center.x, center.y).unwrap();
                        },
                        _ => (),
                    }
                }
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
            .with_title(format!("Damage Report"))
            .build_vk_surface(&events_loop, instance)
            .unwrap();
        self.data = Some(WinitData {
            window,
            events_loop,
            size,
        });
        self.data.as_ref().unwrap().window.surface().clone()
    }
}

pub struct WinitData {
    window: VulkanWinWindow,
    events_loop: EventsLoop,
    size: Vector2<u32>,
}
