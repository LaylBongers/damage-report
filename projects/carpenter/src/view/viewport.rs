use cgmath::{Vector2, Vector3, Quaternion, Rad, Zero, Euler, Angle, InnerSpace};
use window::{AdvancedWindow};

use calcium_rendering::{Error, Renderer, Texture, TextureFormat, Viewport, WindowRenderer};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer, Entity, Material, World3DRenderTarget, Vertex, Mesh};

use carpenter_model::map::{Brush};
use carpenter_model::input::{InputModel, ButtonState};
use carpenter_model::{MapEditor, MapEditorEvent, BusReader};

pub struct ViewportView<R: Renderer, WR: World3DRenderer<R>> {
    render_world: RenderWorld<R, WR>,
    events: BusReader<MapEditorEvent>,

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

            material,

            move_button_started_over_ui: false,
            camera_position: Vector3::new(0.0, 2.0, 5.0),
            camera_pitch: 0.0,
            camera_yaw: 0.0,
        })
    }

    pub fn update<W: AdvancedWindow>(
        &mut self, delta: f32, editor: &MapEditor, input: &InputModel, renderer: &R, window: &mut W
    ) {
        // Check if we got events
        while let Some(ev) = self.events.try_recv() {
            match ev {
                MapEditorEvent::NewBrush(index) => {
                    self.add_brush(editor.brush(index), renderer)
                },
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
        if input.camera_move.state == ButtonState::Released {
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
        if input.forward.state == ButtonState::Pressed { axes.y += 1.0; }
        if input.backward.state == ButtonState::Pressed { axes.y -= 1.0; }
        if input.left.state == ButtonState::Pressed { axes.x -= 1.0; }
        if input.right.state == ButtonState::Pressed { axes.x += 1.0; }
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

    fn add_brush(&mut self, brush: &Brush, renderer: &R) {
        // Create a list of indices and vertices from the brush, we can't merge on indices because
        // every brush face will always have hard edges
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for plane in &brush.planes {
            let normal = plane.normal(brush);

            // Fan-triangulage the face
            // TODO: Optionally support concave faces
            let fan_anchor = brush.vertices[plane.indices[0]];
            let mut last_vertex = brush.vertices[plane.indices[1]];
            for index in plane.indices.iter().skip(2) {
                let vertex = brush.vertices[*index];
                let indices_start = vertices.len() as u32;

                vertices.push(Vertex {
                    position: fan_anchor,
                    normal,
                    uv: Vector2::new(0.0, 0.0),
                });
                vertices.push(Vertex {
                    position: last_vertex,
                    normal,
                    uv: Vector2::new(0.0, 0.0),
                });
                vertices.push(Vertex {
                    position: vertex,
                    normal,
                    uv: Vector2::new(0.0, 0.0),
                });
                // TODO: We can re-use indices here on the same face
                indices.push(indices_start);
                indices.push(indices_start + 1);
                indices.push(indices_start + 2);

                last_vertex = vertex;
            }
        }

        // Now upload the vertices into a mesh and create the world entity
        let mesh = Mesh::new(renderer, vertices, indices);
        self.render_world.add_entity(Entity {
            position: Vector3::new(0.0, 0.0, 0.0),
            mesh: mesh,
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
