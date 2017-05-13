extern crate cgmath;
extern crate cobalt_rendering;
extern crate cobalt_utils;

use cgmath::{Vector3};
use cobalt_rendering::world3d::{Renderer, Camera};
use cobalt_rendering::{Target};
use cobalt_utils::{LoopTimer};

fn main() {
    // Initialize the rendering system
    let target = Target::init();
    let renderer = Renderer::init(target.context());

    // Initialize generic utilities
    let mut timer = LoopTimer::start();

    let mut accumulator = 0.0;

    loop {
        let time = timer.tick();
        accumulator += time;

        // Handle the events that happened to the target
        if !target.poll_events() { break; }

        // Set up the camera to render with
        let camera = Camera {
            position: Vector3::new(accumulator.sin(), 0.0, 1.0),
        };

        // Render the frame
        let mut frame = target.start_frame();
        renderer.render(target.context(), &mut frame, &camera);
        frame.finish().unwrap();
    }
}
