use cgmath::{Vector2};
use glium::backend::glutin_backend::GlutinFacade;
use glium::backend::{Facade};
use glium::glutin::{WindowBuilder, Event as GlutinEvent, ElementState, VirtualKeyCode, ScanCode};
use glium::{DisplayBuild, Surface, Frame as GliumFrame};

pub struct Target {
    context: GlutinFacade,
    size: Vector2<u32>,
}

impl Target {
    pub fn init() -> Self {
        let size = Vector2::new(1280, 720);
        let context = WindowBuilder::new()
            .with_dimensions(size.x, size.y)
            .with_title(format!("Cobalt"))
            .build_glium()
            .unwrap();

        Target {
            context,
            size,
        }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        let mut event = Vec::new();

        for ev in self.context.poll_events() {
            match ev {
                GlutinEvent::Resized(width, height) =>
                    self.size = Vector2::new(width, height),
                GlutinEvent::Closed => event.push(Event::Closed),
                GlutinEvent::KeyboardInput(state, scan_code, key_code) =>
                    event.push(Event::KeyboardInput(state, scan_code, key_code)),
                _ => ()
            }
        }

        event
    }

    pub fn start_frame(&self) -> Frame {
        let mut frame = self.context.draw();

        frame.clear_color(0.005, 0.005, 0.006, 1.0);

        Frame {
            inner: frame,
            size: self.size,
        }
    }

    pub fn context(&self) -> &Facade {
        &self.context
    }
}

pub struct Frame {
    pub inner: GliumFrame,
    pub size: Vector2<u32>,
}

impl Frame {
    pub fn finish(self) -> Result<(), ()> {
        self.inner.finish().map_err(|_| ())
    }
}

#[derive(Debug)]
pub enum Event {
    Closed,
    KeyboardInput(ElementState, ScanCode, Option<VirtualKeyCode>),
}
