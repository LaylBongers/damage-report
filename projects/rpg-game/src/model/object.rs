use cgmath::{Point2, Vector2};

pub struct Object {
    position: Point2<f32>,
    size: Vector2<f32>,
}

impl Object {
    pub fn new(position: Point2<f32>, size: Vector2<f32>) -> Self {
        Object {
            position,
            size,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn size(&self) -> Vector2<f32> {
        self.size
    }
}
