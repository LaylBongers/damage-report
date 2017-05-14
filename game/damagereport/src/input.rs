use cgmath::{Vector2, Zero};
use cobalt_rendering::{ElementState, VirtualKeyCode};

#[derive(Default)]
pub struct FrameInput {
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Default)]
pub struct InputState {
    pub escape_pressed: bool,
    move_right: bool,
    move_left: bool,
    move_forward: bool,
    move_backward: bool,
}

impl InputState {
    pub fn handle_key(&mut self, key_state: ElementState, key_code: VirtualKeyCode) {
        let new_state = key_state == ElementState::Pressed;

        match key_code {
            VirtualKeyCode::Escape => if new_state { self.escape_pressed = true },
            VirtualKeyCode::D => self.move_right = new_state,
            VirtualKeyCode::A => self.move_left = new_state,
            VirtualKeyCode::W => self.move_forward = new_state,
            VirtualKeyCode::S => self.move_backward = new_state,
            _ => (),
        }
    }

    pub fn movement_axes(&self) -> Vector2<f32> {
        let mut direction = Vector2::zero();

        if self.move_right {
            direction += Vector2::new(1.0, 0.0);
        }
        if self.move_left {
            direction -= Vector2::new(1.0, 0.0);
        }
        if self.move_forward {
            direction += Vector2::new(0.0, 1.0);
        }
        if self.move_backward {
            direction -= Vector2::new(0.0, 1.0);
        }

        direction
    }
}
