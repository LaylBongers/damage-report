use cgmath::{Vector2};

#[derive(PartialEq, Clone, Debug)]
pub struct Viewport {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl Viewport {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        Viewport {
            position, size,
        }
    }
}
