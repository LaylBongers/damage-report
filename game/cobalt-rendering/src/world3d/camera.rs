use cgmath::{Vector3, Matrix4, SquareMatrix};

pub struct Camera {
    pub position: Vector3<f32>
}

impl Camera {
    pub fn create_world_to_view_matrix(&self) -> Matrix4<f32> {
        self.create_view_to_world_matrix().invert().unwrap()
    }

    pub fn create_view_to_world_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
    }
}
