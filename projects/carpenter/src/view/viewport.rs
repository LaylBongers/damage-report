use cgmath::{Vector2, Vector3, Quaternion, Rad, Zero, Euler, Angle, InnerSpace};
use window::{AdvancedWindow};

use calcium_rendering::{Error, Renderer, Texture, TextureFormat, Viewport, WindowRenderer};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer, Entity, Model, Material, World3DRenderTarget};

use carpenter_model::{MapEditor, MapEditorEvent, InputModel, BusReader};

pub struct ViewportView<R: Renderer, WR: World3DRenderer<R>> {
    render_world: RenderWorld<R, WR>,
    events: BusReader<MapEditorEvent>,

    model: Model<R, WR>,
    material: Material<R>,

    move_button_started_over_ui: bool,
    camera_position: Vector3<f32>,
    camera_pitch: f32,
    camera_yaw: f32,
}

impl<R: Renderer, WR: World3DRenderer<R>> ViewportView<R, WR> {
    pub fn new(renderer: &mut R, editor: &mut MapEditor) -> Result<Self, Error> {
        let mut render_world = RenderWorld::new();

        render_world.ambient_light = Vector3::new(0.05, 0.05, 0.05);
        render_world.directional_light = Vector3::new(1.0, 1.0, 1.0);

        let model = Model::<R, WR>::load(renderer, "./assets/cube.obj", 1.0);
        let material = Material {
            base_color: Texture::from_file(
                renderer, "./assets/texture.png", TextureFormat::Srgb
            )?,
            normal_map: Texture::from_file(
                renderer, "./assets/texture_normal.png", TextureFormat::Linear
            )?,
            metallic_map: Texture::from_file(
                renderer, "./assets/texture_metallic.png", TextureFormat::LinearRed
            )?,
            roughness_map: Texture::from_file(
                renderer, "./assets/texture_roughness.png", TextureFormat::LinearRed
            )?,
        };

        Ok(ViewportView {
            render_world,
            events: editor.subscribe(),

            model,
            material,

            move_button_started_over_ui: false,
            camera_position: Vector3::new(0.0, 2.0, 5.0),
            camera_pitch: 0.0,
            camera_yaw: 0.0,
        })
    }

    pub fn update<W: AdvancedWindow>(
        &mut self, delta: f32, input: &InputModel, window: &mut W
    ) {
        // Check if we got events
        while let Some(ev) = self.events.try_recv() {
            match ev {
                MapEditorEvent::NewBrush =>
                    self.add_brush(),
            }
        }

        // Update the camera based on input
        self.update_camera(delta, input, window);
    }

    pub fn render(
        &self,
        frame: &mut R::Frame,
        renderer: &mut R,
        window_renderer: &mut R::WindowRenderer,
        world3d_renderer: &mut WR,
        world3d_rendertarget: &mut World3DRenderTarget<R, WR>,
    ) {
        // Create a viewport that doesn't overlap the UI
        let viewport = Viewport::new(
            Vector2::new(0.0, 102.0),
            window_renderer.size().cast() - Vector2::new(0.0, 102.0),
        );

        world3d_renderer.render(
            &self.render_world, &self.create_camera(),
            world3d_rendertarget, &viewport,
            renderer, window_renderer, frame
        );
    }

    fn update_camera<W: AdvancedWindow>(
        &mut self, delta: f32, input: &InputModel, window: &mut W
    ) {
        if !input.camera_move_button {
            window.set_capture_cursor(false);
            self.move_button_started_over_ui = false;

            // We don't need to do anything more
            return;
        }

        // If the move button was started over UI, don't do anything
        if self.move_button_started_over_ui {
            return;
        }

        // Check if we're over ui and if so, mark the button as started over UI
        if input.cursor_over_ui {
            self.move_button_started_over_ui = true;
            return;
        }

        // Finally, we've verified that we've got a real move input
        window.set_capture_cursor(true);

        // Rotate the player's yaw depending on input
        let frame_input = input.frame();
        self.camera_yaw += frame_input.mouse_x * -0.00025;
        self.camera_pitch += frame_input.mouse_y * -0.00025;

        // Limit the pitch
        if self.camera_pitch > 0.25 {
            self.camera_pitch = 0.25;
        }
        if self.camera_pitch < -0.25 {
            self.camera_pitch = -0.25;
        }

        // Calculate the current total WASD axes input
        let mut axes: Vector2<f32> = Vector2::zero();
        if input.forward_button { axes.y += 1.0; }
        if input.backward_button { axes.y -= 1.0; }
        if input.left_button { axes.x -= 1.0; }
        if input.right_button { axes.x += 1.0; }
        if axes == Vector2::zero() {
            return;
        }
        axes = axes.normalize();

        // Rotate the movement to relative to the camera
        let rotation = self.create_camera_rotation();
        let rotated_movement = rotation * Vector3::new(axes.x, 0.0, -axes.y);

        // Apply the final movement
        self.camera_position += rotated_movement * delta * 10.0;

    }

    fn add_brush(&mut self) {
        self.render_world.add_entity(Entity {
            position: Vector3::new(0.0, 0.0, 0.0),
            mesh: self.model.meshes[0].clone(),
            material: self.material.clone(),
        });
    }


    pub fn create_camera(&self) -> Camera {
        Camera {
            position: self.camera_position,
            rotation: self.create_camera_rotation(),
        }
    }

    fn create_camera_rotation(&self) -> Quaternion<f32> {
        let yaw: Quaternion<f32> =
            Euler::new(Rad::zero(), Rad::full_turn() * self.camera_yaw, Rad::zero()).into();
        let pitch: Quaternion<f32> =
            Euler::new(Rad::full_turn() * self.camera_pitch, Rad::zero(), Rad::zero()).into();
        yaw * pitch
    }
}
