use cgmath::{Vector2, Zero};
use calcium_rendering_simple2d::{Rectangle};
use style::{Style};

pub struct Element {
    pub style: Style,
    pub(crate) positioning: Positioning,
    pub(crate) hovering: bool,
    pub(crate) clicked: bool,
}

impl Element {
    pub fn new(style: Style) -> Self {
        Element {
            style,
            positioning: Positioning::new(),
            hovering: false,
            clicked: false,
        }
    }

    pub fn hovering(&self) -> bool {
        self.hovering
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
