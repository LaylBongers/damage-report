use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface};
use vulkano_win::{self, VkSurfaceBuild, Window as VulkanWinWindow};
use winit::{EventsLoop, WindowBuilder, Event, WindowEvent};

use calcium_rendering_vulkano::{WindowCreator, Window};

use input::{InputState, FrameInput};

pub struct VulkanWinWindowCreator;

impl WindowCreator for VulkanWinWindowCreator {
    type W = VulkanWinWindowWrapper;

    fn required_extensions(&self) -> InstanceExtensions {
        vulkano_win::required_extensions()
    }

    fn create_window(&self, instance: Arc<Instance>, size: Vector2<u32>) -> Self::W {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_dimensions(size.x, size.y)
            .with_title(format!("Damage Report"))
            .build_vk_surface(&events_loop, instance)
            .unwrap();
        VulkanWinWindowWrapper { window, events_loop, size }
    }
}

pub struct VulkanWinWindowWrapper {
    window: VulkanWinWindow,
    events_loop: EventsLoop,
    size: Vector2<u32>,
}

impl Window for VulkanWinWindowWrapper {
    fn surface(&self) -> &Arc<Surface> {
        self.window.surface()
    }
}

impl VulkanWinWindowWrapper {
    pub fn handle_events(
        &mut self,
        input_state: &mut InputState, frame_input: &mut FrameInput
    ) -> bool {
        let mut should_continue = true;

        self.events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: ev, .. } => {
                    match ev {
                        WindowEvent::Closed => should_continue = false,
                        WindowEvent::KeyboardInput(key_state, _, Some(key_code), _) =>
                            input_state.handle_key(key_state, key_code),
                        WindowEvent::MouseMoved(x, y) => {
                            let center = (self.size/2).cast();

                            // Check how far away from the center we are and use that to calculate input
                            let difference: Vector2<i32> = Vector2::new(x, y) - center;
                            frame_input.pitch += difference.y as f32 * -0.0005;
                            frame_input.yaw += difference.x as f32 * -0.0005;

                            // Re-center the mouse so it stays in the middle of the screen
                            self.window.window().set_cursor_position(center.x, center.y).unwrap();
                        },
                        _ => (),
                    }
                }
            }
        });

        should_continue
    }
}
