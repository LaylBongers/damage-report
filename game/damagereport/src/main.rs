extern crate cobalt_rendering;

use cobalt_rendering::world3d::{Renderer};
use cobalt_rendering::{Target};

fn main() {
    let target = Target::init();
    let renderer = Renderer::init(target.context());

    loop {
        if !target.poll_events() { break; }

        let mut frame = target.start_frame();

        renderer.render(target.context(), &mut frame);

        frame.finish().unwrap();
    }
}
