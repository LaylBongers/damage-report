use cgmath::{Vector2};
use style::{Style};

pub struct Element {
    pub style: Style,
    pub positioning: Positioning,
}

impl Element {
    pub fn new(style: Style) -> Self {
        Element {
            style,
            positioning: Positioning::new(),
        }
    }

    pub fn clicked(&self) -> bool {
        false
    }
}

pub struct Positioning {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl Positioning {
    pub fn new() -> Self {
        Positioning {
            position: Vector2::new(0.0, 0.0),
            size: Vector2::new(0.0, 0.0),
        }
    }
}
