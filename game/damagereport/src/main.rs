extern crate cgmath;
extern crate cobalt_rendering;
extern crate cobalt_utils;

use cgmath::{Vector2, Vector3, Euler, Rad, Zero, InnerSpace, Angle, Quaternion};
use cobalt_rendering::world3d::{Renderer, Camera};
use cobalt_rendering::{Target, Event, ElementState, VirtualKeyCode};
use cobalt_utils::{LoopTimer};

fn main() {
    // Initialize the rendering system
    let mut target = Target::init();
    let renderer = Renderer::init(target.context());

    // Initialize generic utilities
    let mut timer = LoopTimer::start();

    // Game state
    let mut input_state = InputState::default();
    let mut player = Player::new();

    // The main game loop
    loop {
        let time = timer.tick();

        // Handle any events in the target
        let mut frame_pitch = 0.0;
        let mut frame_yaw = 0.0;
        if !handle_events(&mut target, &mut input_state, &mut frame_pitch, &mut frame_yaw) ||
           input_state.escape_pressed {
            break
        }

        // Update the player based on the input we got so far
        player.update(&input_state, frame_pitch, frame_yaw, time);

        // Perform the actual rendering
        let camera = player.create_camera();
        render_frame(&target, &renderer, &camera);
    }
}

struct Player {
    position: Vector3<f32>,
    pitch: f32,
    yaw: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            position: Vector3::new(0.0, 0.0, 4.0),
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    fn update(&mut self, input_state: &InputState, frame_pitch: f32, frame_yaw: f32, time: f32) {
        // Rotate the player's yaw depending on input
        self.pitch += frame_pitch;
        self.yaw += frame_yaw;

        // Limit the pitch
        if self.pitch > 0.25 {
            self.pitch = 0.25;
        }
        if self.pitch < -0.25 {
            self.pitch = -0.25;
        }

        // Move the player following the movement input, in the direction the player's pointing
        let rotation = self.create_rotation();
        let axes = input_state.movement_axes();
        let mut rotated_movement = rotation * Vector3::new(axes.x, 0.0, -axes.y);
        rotated_movement.y = 0.0;
        self.position += rotated_movement * time;
    }

    fn create_rotation(&self) -> Quaternion<f32> {
        let yaw: Quaternion<f32> =
            Euler::new(Rad::zero(), Rad::full_turn() * self.yaw, Rad::zero()).into();
        let pitch: Quaternion<f32> =
            Euler::new(Rad::full_turn() * self.pitch, Rad::zero(), Rad::zero()).into();
        yaw * pitch
    }

    fn create_camera(&self) -> Camera {
        Camera {
            position: self.position + Vector3::new(0.0, 1.6, 0.0),
            rotation: self.create_rotation(),
        }
    }
}

fn handle_events(
    target: &mut Target, input_state: &mut InputState,
    frame_pitch: &mut f32, frame_yaw: &mut f32
) -> bool {
    let mut should_continue = true;

    for event in target.poll_events() {
        match event {
            Event::Closed => should_continue = false,
            Event::KeyboardInput(key_state, _, Some(key_code)) =>
                input_state.handle_key(key_state, key_code),
            Event::MouseMoved(position) => {
                let center = target.size()/2;

                // Check how far away from the center we are and use that to calculate input
                let difference: Vector2<i32> = position.cast() - center.cast();
                *frame_pitch += difference.y as f32 * -0.0005;
                *frame_yaw += difference.x as f32 * -0.0005;

                // Re-center the mouse so it stays in the middle of the screen
                target.set_cursor_position(center);
            },
            _ => (),
        }
    }

    should_continue
}

fn render_frame(target: &Target, renderer: &Renderer, camera: &Camera) {
    let mut frame = target.start_frame();
    renderer.render(target.context(), &mut frame, camera);
    frame.finish().unwrap();
}

#[derive(Default)]
struct InputState {
    pub escape_pressed: bool,
    move_right: bool,
    move_left: bool,
    move_forward: bool,
    move_backward: bool,
}

impl InputState {
    fn handle_key(&mut self, key_state: ElementState, key_code: VirtualKeyCode) {
        let new_state = key_state == ElementState::Pressed;

        match key_code {
            VirtualKeyCode::Escape => if new_state { self.escape_pressed = true },
            VirtualKeyCode::D => self.move_right = new_state,
            VirtualKeyCode::A => self.move_left = new_state,
            VirtualKeyCode::W => self.move_forward = new_state,
            VirtualKeyCode::S => self.move_backward = new_state,
            _ => (),
        }
    }

    fn movement_axes(&self) -> Vector2<f32> {
        let mut direction = Vector2::zero();

        if self.move_right {
            direction += Vector2::new(1.0, 0.0);
        }
        if self.move_left {
            direction -= Vector2::new(1.0, 0.0);
        }
        if self.move_forward {
            direction += Vector2::new(0.0, 1.0);
        }
        if self.move_backward {
            direction -= Vector2::new(0.0, 1.0);
        }

        if direction != Vector2::zero() {
            direction.normalize()
        } else {
            Vector2::zero()
        }
    }
}
