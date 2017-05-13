use cgmath::{Vector3, Matrix4, SquareMatrix, Quaternion};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Camera {
    pub fn create_world_to_view_matrix(&self) -> Matrix4<f32> {
        self.create_view_to_world_matrix().invert().unwrap()
    }

    pub fn create_view_to_world_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(self.position);
        let rotation: Matrix4<f32> = self.rotation.into();
        translation * rotation
    }
}
