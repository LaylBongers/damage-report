use cgmath::{self, Vector2, Point2, Matrix4, Vector3};

/// Defines how the coordinates in render batches will be translated to the screen.
pub enum Projection {
    Pixels,
    Camera(Camera),
}

impl Projection {
    pub fn to_matrix(&self, target_size: Vector2<u32>) -> Matrix4<f32> {
        match *self {
            Projection::Pixels => {
                cgmath::ortho(
                    0.0, target_size.x as f32,
                    target_size.y as f32, 0.0,
                    1.0, -1.0
                )
            },
            Projection::Camera(ref camera) => {
                camera.to_matrix(target_size)
            },
        }
    }
}

/// A definition of a 2D camera.
pub struct Camera {
    pub pixels_per_unit: f32,
    pub position: Point2<f32>,
}

impl Camera {
    pub fn new(pixels_per_unit: f32, position: Point2<f32>) -> Camera {
        Camera {
            pixels_per_unit,
            position,
        }
    }

    pub fn to_matrix(&self, target_size: Vector2<u32>) -> Matrix4<f32> {
        let half_size = target_size.cast() / self.pixels_per_unit / 2.0;
        let projection = cgmath::ortho(
            -half_size.x, half_size.x,
            -half_size.y, half_size.y,
            1.0, -1.0
        );
        let view = Matrix4::from_translation(Vector3::new(-self.position.x, -self.position.y, 0.0));

        projection * view
    }
}
