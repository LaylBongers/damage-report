extern crate cgmath;
extern crate cobalt_rendering;
extern crate cobalt_utils;

mod input;
mod player;

use cgmath::{Vector2};
use cobalt_rendering::world3d::{Renderer, Camera};
use cobalt_rendering::{Target, Event};
use cobalt_utils::{LoopTimer};

use input::{InputState, FrameInput};
use player::{Player};

fn main() {
    // Initialize the rendering system
    let mut target = Target::init();
    let renderer = Renderer::init(target.context());

    // Initialize generic utilities
    let mut timer = LoopTimer::start();

    // Game state
    let mut input_state = InputState::default();
    let mut player = Player::new();

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
        render_frame(&target, &renderer, &camera);
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
            Event::KeyboardInput(key_state, _, Some(key_code)) =>
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

fn render_frame(target: &Target, renderer: &Renderer, camera: &Camera) {
    let mut frame = target.start_frame();
    renderer.render(target.context(), &mut frame, camera);
    frame.finish().unwrap();
}
