extern crate cgmath;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate calcium_rendering;
extern crate calcium_rendering_world3d;
extern crate calcium_utils;
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod game_world;
mod input;
mod player;
mod window;

use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use calcium_rendering::{Error, Target};
use calcium_rendering_world3d::{Renderer, Camera, World};
use calcium_utils::{LoopTimer};

use game_world::{GameWorld};
use input::{InputState, FrameInput};
use window::{VulkanWinWindowCreator};

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
        let should_continue = window.handle_events(&mut input_state, &mut frame_input);
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

fn render_frame(target: &mut Target, renderer: &mut Renderer, camera: &Camera, world: &World) {
    // Start the frame
    let mut frame = target.start_frame();

    // Render the world itself
    renderer.render(target, &mut frame, camera, world);

    // Finish the frame
    target.finish_frame(frame);
}
