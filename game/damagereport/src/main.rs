extern crate cgmath;
extern crate cobalt_rendering;
extern crate cobalt_utils;

use cgmath::{Vector2, Vector3, Euler, Rad, Zero, InnerSpace, Angle};
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
    while handle_events(&mut target, &mut input_state) {
        let time = timer.tick();

        // Update the player based on the input we got so far
        player.update(&input_state, time);

        // Perform the actual rendering
        let camera = player.create_camera();
        render_frame(&target, &renderer, &camera);
    }
}

struct Player {
    position: Vector3<f32>,
    yaw: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            position: Vector3::new(0.0, 0.0, 4.0),
            yaw: 0.0,
        }
    }

    fn update(&mut self, input_state: &InputState, time: f32) {
        let axes = input_state.movement_axes();
        self.position += Vector3::new(axes.x, 0.0, -axes.y) * time;
    }

    fn create_camera(&self) -> Camera {
        Camera {
            position: self.position + Vector3::new(0.0, 1.6, 0.0),
            rotation: Euler::new(
                Rad::zero(), Rad::full_turn() * self.yaw, Rad::zero(),
            ).into(),
        }
    }
}

fn handle_events(target: &mut Target, input_state: &mut InputState) -> bool {
    let mut should_continue = true;

    for event in target.poll_events() {
        match event {
            Event::Closed => should_continue = false,
            Event::KeyboardInput(key_state, _, Some(key_code)) =>
                input_state.handle_key(key_state, key_code),
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
    move_right: bool,
    move_left: bool,
    move_forward: bool,
    move_backward: bool,
}

impl InputState {
    fn handle_key(&mut self, key_state: ElementState, key_code: VirtualKeyCode) {
        let new_state = key_state == ElementState::Pressed;

        match key_code {
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
