use cgmath::{self, Vector2, Matrix4};

/// Defines how the coordinates in render batches will be translated to the screen.
pub enum Projection {
    Pixels,
}

impl Projection {
    pub fn to_matrix(&self, target_size: Vector2<u32>) -> Matrix4<f32> {
        cgmath::ortho(
            0.0, target_size.x as f32,
            target_size.y as f32, 0.0,
            1.0, -1.0
        )
    }
}
