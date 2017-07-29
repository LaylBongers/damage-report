use cgmath::{Vector3, Matrix4, SquareMatrix, Quaternion, Rad, Angle};

use calcium_rendering::{Viewport};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, rotation: Quaternion<f32>) -> Self {
        Camera {
            position: position,
            rotation: rotation,
        }
    }

    pub fn world_to_view_matrix(&self) -> Matrix4<f32> {
        self.view_to_world_matrix().invert().unwrap()
    }

    pub fn view_to_world_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(self.position);
        let rotation: Matrix4<f32> = self.rotation.into();
        translation * rotation
    }

    pub fn screen_to_world_matrix(&self, viewport: &Viewport) -> Matrix4<f32> {
        self.world_to_screen_matrix(viewport).invert().unwrap()
    }

    pub fn world_to_screen_matrix(&self, viewport: &Viewport) -> Matrix4<f32> {
        // Create the projection matrix, which is what makes this a 3D perspective renderer
        let y_fov = Rad::full_turn() * 0.1638; // 90 deg x-fov for this aspect ratio
        let aspect = viewport.size.x / viewport.size.y;
        let projection = create_infinity_projection(y_fov, aspect, 0.1);

        // Combine the projection and the view, we don't need them separately
        let view = self.world_to_view_matrix();
        projection * view
    }
}

/// This projection function creates a "Reverse-Z Infinity Far Plane" projection. It has various
/// advantages over a traditional forward Z near/far projection.
///
/// The reverse Z improves precision on floating point depth buffers, because the Z in projection
/// matrices isn't linear, values near the camera will take up a lot more of the number line than
/// values far away will. Reverse-Z allows values far away to use floating point values closer to
/// zero, taking advantage of the ability of floating point values to adjust precision. This will
/// give identical results for integer depth buffers, so we might as well make use of it.
///
/// The infinity far plane makes it much easier to create games with extremely long view distances.
/// It also means you don't actually have to worry about the far clipping plane removing things
/// you want on screen.
///
/// This projection matrix gives depth values in the 0..1 range, and Y values matching Vulkan's
/// screen space (Y is down).
fn create_infinity_projection(y_fov: Rad<f32>, aspect: f32, z_near: f32) -> Matrix4<f32> {
    let f = 1.0 / (y_fov.0 / 2.0).tan();
    Matrix4::new(
        f / aspect, 0.0,  0.0,  0.0,
        0.0, -f, 0.0, 0.0,
        0.0, 0.0, 0.0, -1.0,
        0.0, 0.0, z_near, 0.0
    )
}
