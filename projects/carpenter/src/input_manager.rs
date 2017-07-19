use input::{Input, Button, MouseButton, Motion};

pub struct InputManager {
    navigate_button: bool,

    frame: FrameInput,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            navigate_button: false,
            frame: FrameInput::new(),
        }
    }

    pub fn navigate_button(&self) -> bool {
        self.navigate_button
    }

    pub fn frame(&self) -> &FrameInput {
        &self.frame
    }

    pub fn new_frame(&mut self) {
        self.frame = FrameInput::new();
    }

    pub fn handle_event(&mut self, event: &Input) {
        match *event {
            Input::Press(Button::Mouse(MouseButton::Right)) =>
                self.navigate_button = true,
            Input::Release(Button::Mouse(MouseButton::Right)) =>
                self.navigate_button = false,
            Input::Move(Motion::MouseRelative(x, y)) => {
                self.frame.mouse_x += x as f32;
                self.frame.mouse_y += y as f32;
            },
            _ => {}
        }
    }
}

pub struct FrameInput {
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl FrameInput {
    pub fn new() -> Self {
        FrameInput {
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}
