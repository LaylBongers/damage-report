use input::{Input, Button, MouseButton};

pub struct InputManager {
    navigate_button: bool,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            navigate_button: false,
        }
    }

    pub fn navigate_button(&self) -> bool {
        self.navigate_button
    }

    pub fn handle_event(&mut self, event: &Input) {
        match event {
            &Input::Press(Button::Mouse(MouseButton::Right)) =>
                self.navigate_button = true,
            &Input::Release(Button::Mouse(MouseButton::Right)) =>
                self.navigate_button = false,
            _ => {}
        }
    }
}
