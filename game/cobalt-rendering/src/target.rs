use cgmath::{Vector2};
use glium::backend::glutin_backend::GlutinFacade;
use glium::backend::{Facade};
use glium::glutin::{WindowBuilder, Event as GlutinEvent, ElementState, VirtualKeyCode, ScanCode};
use glium::{DisplayBuild, Surface, Frame as GliumFrame};

pub struct Target {
    context: GlutinFacade,
    size: Vector2<u32>,
    focused: bool,
}

impl Target {
    pub fn init() -> Self {
        let size = Vector2::new(1280, 720);
        let context = WindowBuilder::new()
            .with_dimensions(size.x, size.y)
            .with_title(format!("Cobalt"))
            .with_depth_buffer(24)
            .build_glium()
            .unwrap();

        Target {
            context,
            size,
            focused: true,
        }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        let mut event = Vec::new();

        for ev in self.context.poll_events() {
            match ev {
                GlutinEvent::Resized(width, height) =>
                    self.size = Vector2::new(width, height),
                GlutinEvent::Closed => event.push(Event::Closed),
                GlutinEvent::Focused(focused) =>
                    self.focused = focused,
                GlutinEvent::KeyboardInput(state, scan_code, key_code) =>
                    event.push(Event::KeyboardInput(state, scan_code, key_code)),
                GlutinEvent::MouseMoved(x, y) =>
                    if self.focused {
                        event.push(Event::MouseMoved(Vector2::new(x as u32, y as u32)))
                    },
                _ => ()
            }
        }

        event
    }

    pub fn start_frame(&self) -> Frame {
        let mut frame = self.context.draw();

        frame.clear_color_and_depth((0.005, 0.005, 0.006, 1.0), 1.0);

        Frame {
            inner: frame,
            size: self.size,
        }
    }

    pub fn set_cursor_position(&self, position: Vector2<u32>) {
        self.context
            .get_window().unwrap()
            .set_cursor_position(position.x as i32, position.y as i32)
            .unwrap();
    }

    pub fn context(&self) -> &Facade {
        &self.context
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
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
    MouseMoved(Vector2<u32>),
}
