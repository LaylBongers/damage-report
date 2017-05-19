extern crate cgmath;
extern crate cobalt_rendering;
extern crate cobalt_utils;

mod input;
mod player;

use cgmath::{Vector2, Vector3};
use cobalt_rendering::world3d::{Renderer, Camera, World, Model};
use cobalt_rendering::{Target, Event};
use cobalt_utils::{LoopTimer};

use input::{InputState, FrameInput};
use player::{Player};

fn main() {
    // Initialize the rendering system
    let mut target = Target::init();
    let renderer = Renderer::init(&target);
    let mut world = World::default();

    // Initialize generic utilities
    let mut timer = LoopTimer::start();

    // Game state
    let mut input_state = InputState::default();
    let mut player = Player::new();

    // Create the floor
    let floor_model = Model::load(&target, "./assets/floor.obj", 0.1);
    world.add(Vector3::new(0.0, 0.0, 0.0), floor_model);

    // Create the 3 test devices
    let device_model = Model::load(&target, "./assets/device.obj", 0.1);
    world.add(Vector3::new(-2.0, 0.0, -4.0), device_model.clone());
    world.add(Vector3::new( 0.0, 0.0, -4.0), device_model.clone());
    world.add(Vector3::new( 2.0, 0.0, -4.0), device_model.clone());

    // The main game loop
    loop {
        let time = timer.tick();

        // Handle any events in the target
        let mut frame_input = FrameInput::default();
        if !handle_events(&mut target, &mut input_state, &mut frame_input) ||
           input_state.escape_pressed {
            break
        }

        // Update the player based on the input we got so far
        player.update(&input_state, &frame_input, time);

        // Perform the actual rendering
        let camera = player.create_camera();
        render_frame(&mut target, &renderer, &camera, &world);
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

fn render_frame(target: &mut Target, renderer: &Renderer, camera: &Camera, world: &World) {
    // Start the frame
    let mut frame = target.start_frame();

    // Render the world itself
    renderer.render(target, &mut frame, camera, world);

    // Finish the frame
    target.finish_frame(frame);
}
