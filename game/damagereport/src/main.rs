extern crate cgmath;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate cobalt_rendering;
extern crate cobalt_utils;

mod game_world;
mod input;
mod player;

use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use cobalt_rendering::world3d::{Renderer, Camera, World};
use cobalt_rendering::{Target, Event};
use cobalt_utils::{LoopTimer};

use game_world::{GameWorld};
use input::{InputState, FrameInput};

fn main() {
    // Set up the logger
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    let init_log = log.new(o!("state" => "initializing"));
    info!(init_log, "Damage Report Version {}", env!("CARGO_PKG_VERSION"));

    // Initialize the rendering system
    let mut target = Target::init(&init_log);
    let mut renderer = Renderer::init(&target);
    let mut world = World::default();

    // Initialize generic utilities
    let mut timer = LoopTimer::start();
    let mut input_state = InputState::default();

    // Initialize the gamae world
    let mut game_world = GameWorld::init(&init_log, &mut target, &mut world);

    // The main game loop
    let _loop_log = log.new(o!("state" => "game loop"));
    loop {
        let time = timer.tick();

        // Handle any events in the target
        let mut frame_input = FrameInput::default();
        if !handle_events(&mut target, &mut input_state, &mut frame_input) ||
           input_state.escape_pressed {
            break
        }

        game_world.update(time, &mut world, &input_state, &frame_input);

        // Perform the actual rendering
        let camera = game_world.player.create_camera();
        render_frame(&mut target, &mut renderer, &camera, &world);
    }
}

fn handle_events(
    target: &mut Target,
    input_state: &mut InputState, frame_input: &mut FrameInput
) -> bool {
    let mut should_continue = true;

    for event in target.poll_events() {
        match event {
            Event::Closed => should_continue = false,
            Event::KeyboardInput(key_state, _, Some(key_code), _) =>
                input_state.handle_key(key_state, key_code),
            Event::MouseMoved(position) => {
                let center = target.size()/2;

                // Check how far away from the center we are and use that to calculate input
                let difference: Vector2<i32> = position.cast() - center.cast();
                frame_input.pitch += difference.y as f32 * -0.0005;
                frame_input.yaw += difference.x as f32 * -0.0005;

                // Re-center the mouse so it stays in the middle of the screen
                target.set_cursor_position(center);
            },
            _ => (),
        }
    }

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
