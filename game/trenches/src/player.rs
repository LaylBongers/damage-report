use cgmath::{Vector3, Euler, Rad, Zero, Angle, Quaternion, InnerSpace, Vector2};
use calcium_rendering_world3d::{Camera};

use input::{InputState, FrameInput};

pub struct Player {
    position: Vector3<f32>,
    pitch: f32,
    yaw: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vector3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn update(&mut self, input_state: &InputState, frame_input: &FrameInput, time: f32) {
        // Rotate the player's yaw depending on input
        self.pitch += frame_input.pitch;
        self.yaw += frame_input.yaw;

        // Limit the pitch
        if self.pitch > 0.25 {
            self.pitch = 0.25;
        }
        if self.pitch < -0.25 {
            self.pitch = -0.25;
        }

        // Get the input we've currently got for movement and make sure we're not standing still
        let axes = input_state.movement_axes();
        if axes == Vector2::zero() {
            return;
        }

        // Rotate the movement to relative to the player
        let yaw: Quaternion<f32> =
            Euler::new(Rad::zero(), Rad::full_turn() * self.yaw, Rad::zero()).into();
        let mut rotated_movement = yaw * Vector3::new(axes.x, 0.0, -axes.y);

        // Remove the Y component of the movement and normalize it
        // We know normalization will work because we early bail if we don't have input
        rotated_movement.y = 0.0;
        let final_movement = rotated_movement.normalize();

        // Finally, apply the final movement
        self.position += final_movement * time;
    }

    pub fn create_camera(&self) -> Camera {
        Camera {
            position: self.position + Vector3::new(0.0, 1.6, 0.0),
            rotation: self.create_camera_rotation(),
        }
    }

    fn create_camera_rotation(&self) -> Quaternion<f32> {
        let yaw: Quaternion<f32> =
            Euler::new(Rad::zero(), Rad::full_turn() * self.yaw, Rad::zero()).into();
        let pitch: Quaternion<f32> =
            Euler::new(Rad::full_turn() * self.pitch, Rad::zero(), Rad::zero()).into();
        yaw * pitch
    }
}
