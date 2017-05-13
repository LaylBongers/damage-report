use glium::backend::glutin_backend::GlutinFacade;
use glium::backend::{Facade};
use glium::glutin::{WindowBuilder, Event};
use glium::{DisplayBuild, Surface, Frame};

pub struct Target {
    context: GlutinFacade,
}

impl Target {
    pub fn init() -> Self {
        let context = WindowBuilder::new()
            .with_dimensions(1280, 720)
            .with_title(format!("Cobalt"))
            .build_glium()
            .unwrap();

        Target {
            context
        }
    }

    pub fn poll_events(&self) -> bool {
        for ev in self.context.poll_events() {
            match ev {
                Event::Closed => return false,
                _ => ()
            }
        }

        true
    }

    pub fn start_frame(&self) -> Frame {
        let mut frame = self.context.draw();

        frame.clear_color(0.005, 0.005, 0.006, 1.0);

        frame
    }

    pub fn context(&self) -> &Facade {
        &self.context
    }
}
