use cgmath::{Vector2};
use palette::{Rgba};

#[derive(Clone, Debug)]
pub struct Style {
    pub position: Position,
    pub margin: Lrtb,
    pub size: Vector2<f32>,

    pub background_color: Option<Rgba>,
    pub text: String,
}

impl Style {
    pub fn new() -> Self {
        Style {
            position: Position::Flow,
            margin: Lrtb::new(0.0, 0.0, 0.0, 0.0),
            size: Vector2::new(0.0, 0.0),

            background_color: None,
            text: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lrtb {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Lrtb {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Lrtb {
            left, right, top, bottom
        }
    }

    pub fn uniform(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn left_top(&self) -> Vector2<f32> {
        Vector2::new(self.left, self.top)
    }
}

#[derive(Clone, Debug)]
pub enum Position {
    /// Positions the element using previous elements.
    Flow,
    /// Positions the element using a position relative to the parent.
    Relative(Vector2<f32>),
}
