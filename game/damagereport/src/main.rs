extern crate cgmath;
extern crate cobalt_rendering;
extern crate cobalt_utils;

use cgmath::{Vector3, Euler, Rad, Zero};
use cobalt_rendering::world3d::{Renderer, Camera};
use cobalt_rendering::{Target, Event};
use cobalt_utils::{LoopTimer};

fn main() {
    // Initialize the rendering system
    let mut target = Target::init();
    let renderer = Renderer::init(target.context());

    // Initialize generic utilities
    let mut timer = LoopTimer::start();

    // Game state
    let player_position = Vector3::new(0.0, 0.0, 4.0);

    // The main game loop
    loop {
        let _time = timer.tick();

        // Handle the events that happened to the target
        for event in target.poll_events() {
            match event {
                Event::Closed => return,
                _ => (),
            }
        }

        // Set up the camera to render with
        let camera = Camera {
            position: player_position + Vector3::new(0.0, 1.6, 0.0),
            rotation: Euler::new(
                Rad::zero(), Rad::zero(), Rad::zero()
            ).into(),
        };

        // Render the frame
        let mut frame = target.start_frame();
        renderer.render(target.context(), &mut frame, &camera);
        frame.finish().unwrap();
    }
}
