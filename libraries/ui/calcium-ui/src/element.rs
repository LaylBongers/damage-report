use cgmath::{Vector2, Zero};
use calcium_rendering_simple2d::{Rectangle};
use style::{Style};

pub struct Element {
    pub style: Style,
    pub text: ElementText,

    // Cached data
    pub(crate) positioning: Positioning,

    // Input state
    pub(crate) cursor_state: ElementCursorState,
    pub(crate) clicked: bool,
}

impl Element {
    pub fn new(style: Style) -> Self {
        Element {
            style,
            text: ElementText::None,

            positioning: Positioning::new(),

            cursor_state: ElementCursorState::None,
            clicked: false,
        }
    }

    pub fn hovering(&self) -> bool {
        self.cursor_state == ElementCursorState::Hovering
    }

    pub fn held(&self) -> bool {
        self.cursor_state == ElementCursorState::Held
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

#[derive(Debug)]
pub struct Positioning {
    pub rectangle: Rectangle<f32>,
}

impl Positioning {
    pub fn new() -> Self {
        Positioning {
            rectangle: Rectangle::new(Vector2::zero(), Vector2::zero()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementCursorState {
    None,
    Hovering,
    Held,
}

#[derive(Debug)]
pub enum ElementText {
    None,
    SingleLine(String),
}
