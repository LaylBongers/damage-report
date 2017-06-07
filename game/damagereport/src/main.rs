extern crate cgmath;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate cobalt_rendering;
extern crate cobalt_rendering_world3d;
extern crate cobalt_utils;
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod game_world;
mod input;
mod player;

use std::sync::{Arc};

use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface};
use vulkano_win::{VkSurfaceBuild, Window};
use winit::{EventsLoop, WindowBuilder, Event, WindowEvent};

use cobalt_rendering::{Error, Target, WindowCreator, WindowRemoveThisPart};
use cobalt_rendering_world3d::{Renderer, Camera, World};
use cobalt_utils::{LoopTimer};

use game_world::{GameWorld};
use input::{InputState, FrameInput};

fn main() {
    // Set up the logger
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    info!(log, "Damage Report Version {}", env!("CARGO_PKG_VERSION"));

    // Run the actual game
    let result = try_main(&log);

    // Check the result of running the game
    if let Err(err) = result {
        error!(log, "{}", err);
    }
}

fn try_main(log: &Logger) -> Result<(), Error> {
    info!(log, "Initializing game");

    // Initialize the rendering system
    let (mut target, mut window) = Target::new(log, VulkanWinWindowCreator)?;
    let mut renderer = Renderer::new(log, &target);
    let mut world = World::new();

    // Initialize generic utilities
    let mut timer = LoopTimer::start();
    let mut input_state = InputState::default();

    // Initialize the game world
    let mut game_world = GameWorld::new(log, &mut target, &mut world);

    // The main game loop
    info!(log, "Starting game loop");
    loop {
        let time = timer.tick();

        // Handle any events in the target
        let mut frame_input = FrameInput::default();
        let should_continue = handle_events(&mut window, &mut input_state, &mut frame_input);
        if !should_continue || input_state.escape_pressed {
            break
        }

        game_world.update(time, &mut world, &input_state, &frame_input);

        // Perform the actual rendering
        let camera = game_world.player.create_camera();
        render_frame(&mut target, &mut renderer, &camera, &world);
    }
    info!(log, "Ending game loop");

    Ok(())
}

fn handle_events(
    window: &mut VulkanWinWindow,
    input_state: &mut InputState, frame_input: &mut FrameInput
) -> bool {
    let mut should_continue = true;

    window.events_loop.poll_events(|event| {
        match event {
            Event::WindowEvent { event: ev, .. } => {
                match ev {
                    WindowEvent::Closed => should_continue = false,
                    WindowEvent::KeyboardInput(key_state, _, Some(key_code), _) =>
                        input_state.handle_key(key_state, key_code),
                    WindowEvent::MouseMoved(x, y) => {
                        let center = (window.size/2).cast();

                        // Check how far away from the center we are and use that to calculate input
                        let difference: Vector2<i32> = Vector2::new(x, y) - center;
                        frame_input.pitch += difference.y as f32 * -0.0005;
                        frame_input.yaw += difference.x as f32 * -0.0005;

                        // Re-center the mouse so it stays in the middle of the screen
                        window.window.window().set_cursor_position(center.x, center.y).unwrap();
                    },
                    _ => (),
                }
            }
        }
    });

    should_continue
}

fn render_frame(target: &mut Target, renderer: &mut Renderer, camera: &Camera, world: &World) {
    // Start the frame
    let mut frame = target.start_frame();

    // Render the world itself
    renderer.render(target, &mut frame, camera, world);

    // Finish the frame
    target.finish_frame(frame);
}

struct VulkanWinWindowCreator;
impl WindowCreator for VulkanWinWindowCreator {
    type W = VulkanWinWindow;

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
        VulkanWinWindow { window, events_loop, size }
    }
}

struct VulkanWinWindow {
    window: Window,
    events_loop: EventsLoop,
    size: Vector2<u32>,
}

impl WindowRemoveThisPart for VulkanWinWindow {
    fn surface(&self) -> &Arc<Surface> {
        self.window.surface()
    }
}
