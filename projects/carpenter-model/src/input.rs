use cgmath::{Vector2};
use pinput::{Input, Button, MouseButton, Motion, Key};

pub struct InputModel {
    /// Generally the left mouse button
    pub primary_action: ButtonModel,
    /// Generally the right mouse button
    pub camera_move: ButtonModel,

    pub forward: ButtonModel,
    pub left: ButtonModel,
    pub backward: ButtonModel,
    pub right: ButtonModel,

    pub cursor_pixel_position: Vector2<f32>,
    pub cursor_over_ui: bool,

    frame: FrameInput,
}

impl InputModel {
    pub fn new() -> Self {
        InputModel {
            primary_action: ButtonModel::new(Button::Mouse(MouseButton::Left)),
            camera_move: ButtonModel::new(Button::Mouse(MouseButton::Right)),

            forward: ButtonModel::new(Button::Keyboard(Key::W)),
            left: ButtonModel::new(Button::Keyboard(Key::A)),
            backward: ButtonModel::new(Button::Keyboard(Key::S)),
            right: ButtonModel::new(Button::Keyboard(Key::D)),

            cursor_pixel_position: Vector2::new(0.0, 0.0),
            cursor_over_ui: false,

            frame: FrameInput::new(),
        }
    }

    pub fn frame(&self) -> &FrameInput {
        &self.frame
    }

    pub fn new_frame(&mut self) {
        self.primary_action.pressed = false;
        self.camera_move.pressed = false;
        self.forward.pressed = false;
        self.left.pressed = false;
        self.backward.pressed = false;
        self.right.pressed = false;

        self.frame = FrameInput::new();
    }

    pub fn handle_event(&mut self, event: &Input) {
        self.primary_action.handle_event(event);
        self.camera_move.handle_event(event);
        self.forward.handle_event(event);
        self.left.handle_event(event);
        self.backward.handle_event(event);
        self.right.handle_event(event);

        match *event {
            Input::Move(Motion::MouseCursor(x, y)) => {
                self.cursor_pixel_position = Vector2::new(x as f32, y as f32);
            },
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

pub struct ButtonModel {
    button: Button,
    pub state: ButtonState,
    pub pressed: bool,
    pub released: bool,
}

impl ButtonModel {
    pub fn new(button: Button) -> Self {
        ButtonModel {
            button,
            state: ButtonState::Released,
            pressed: false,
            released: false,
        }
    }

    pub fn handle_event(&mut self, event: &Input) {
        match *event {
            Input::Press(button) => {
                if button == self.button {
                    self.state = ButtonState::Pressed;
                    self.pressed = true;
                }
            },
            Input::Release(button) => {
                if button == self.button {
                    self.state = ButtonState::Released;
                    self.released = true;
                }
            },
            _ => {},
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Pressed,
    Released,
}
