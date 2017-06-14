extern crate cgmath;
extern crate calcium;
extern crate calcium_rendering;
extern crate calcium_rendering_vulkano;
extern crate calcium_rendering_world3d;
extern crate calcium_rendering_world3d_vulkano;
extern crate calcium_game;
extern crate noise;
extern crate num;
extern crate rayon;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod game_world;
mod input;
mod player;
mod target;
mod voxel_grid;

use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use calcium::rendering::{self, Backend};
use calcium_game::{LoopTimer};
use calcium_rendering::{Error, RenderSystem};
use calcium_rendering_vulkano::{VulkanoRenderBackend};
use calcium_rendering_world3d::{RenderWorld, WorldRenderSystem};
use calcium_rendering_world3d_vulkano::{VulkanoWorldRenderBackend};

use game_world::{GameWorld};
use input::{InputState, FrameInput};
use target::{WinitTargetSystem};

fn main() {
    // Set up the logger
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    info!(log, "Damage Report Version {}", env!("CARGO_PKG_VERSION"));

    // Run the actual game
    let result = run_game(&log);

    // Check the result of running the game
    if let Err(err) = result {
        error!(log, "{}", err);
    }
}

fn run_game(log: &Logger) -> Result<(), Error> {
    info!(log, "Initializing game");

    // TODO: Read in from configuration and UI
    let backend = Backend::Vulkano;

    // Regardless of what backend we're using right now, it will always have a winit window, but
    //  it depends on the backend how it should be initialized. For this reason we use a
    //  Window System which implements the initialization traits required by the backends.
    let mut window_system = WinitTargetSystem::new();

    // Create the backends and render system based on what we were told to
    let (mut render_system, mut world_render_system) = rendering::new_renderer_systems(
        log, backend, &mut window_system
    )?;

    // Initialize generic utilities
    let mut timer = LoopTimer::start();
    let mut input_state = InputState::default();

    // Initialize the game world
    let mut render_world = RenderWorld::new();
    let mut game_world = GameWorld::new(log, &mut render_world);

    // The main game loop
    info!(log, "Starting game loop");
    loop {
        let time = timer.tick();

        // Handle any events in the target
        let mut frame_input = FrameInput::default();
        let should_continue = window_system.handle_events(&mut input_state, &mut frame_input);
        if !should_continue || input_state.escape_pressed {
            break
        }

        // Update the gameworld itself
        game_world.update(time, &mut render_world, &input_state, &frame_input);

        // Perform the actual rendering
        let camera = game_world.player.create_camera();
        let mut frame = render_system.start_frame();
        world_render_system.render(
            log, render_system.as_mut(), frame.as_mut(), &camera, &render_world
        );
        render_system.finish_frame(frame);
    }
    info!(log, "Ending game loop");

    Ok(())
}
