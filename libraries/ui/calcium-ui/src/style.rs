use cgmath::{Vector2};

pub struct Style {
    pub position: Position,
    pub margin: Vector2<f32>,
    pub size: Vector2<f32>,
    pub text: String,
}

impl Style {
    pub fn new() -> Self {
        Style {
            position: Position::Flow,
            margin: Vector2::new(0.0, 0.0),
            size: Vector2::new(0.0, 0.0),
            text: String::new(),
        }
    }
}

pub enum Position {
    /// Positions the element using previous elements.
    Flow,
    /// Positions the element using a position relative to the parent.
    Relative(Vector2<f32>),
}
