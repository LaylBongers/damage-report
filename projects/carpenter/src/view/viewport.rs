use slog::{Logger};
use cgmath::prelude::*;
use cgmath::{Point2, Point3, Vector2, Vector3, Quaternion, Rad, Euler};
use window::{AdvancedWindow};
use input::{ButtonState};
use collision::{Ray3, Plane};

use calcium_rendering::{Error, Renderer, Texture, TextureFormat, Viewport, WindowRenderer};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer, Entity, Material, World3DRenderTarget, Vertex, Mesh};

use carpenter_model::map::{Brush};
use carpenter_model::input::{InputModel};
use carpenter_model::{MapEditor, MapEditorEvent, BusReader};

pub struct ViewportView<R: Renderer, WR: World3DRenderer<R>> {
    render_world: RenderWorld<R, WR>,
    material: Material<R>,
    last_viewport: Viewport,
    events: BusReader<MapEditorEvent>,

    move_button_started_over_ui: bool,
    camera_position: Vector3<f32>,
    camera_pitch: f32,
    camera_yaw: f32,
}

impl<R: Renderer, WR: World3DRenderer<R>> ViewportView<R, WR> {
    pub fn new(renderer: &mut R, editor: &mut MapEditor) -> Result<Self, Error> {
        let mut render_world = RenderWorld::new();

        render_world.ambient_light = Vector3::new(0.05, 0.05, 0.05);
        render_world.directional_light = Vector3::new(1.0, 1.0, 1.0) * 2.5;
        render_world.directional_direction = Vector3::new(1.0, 2.0, 1.5).normalize();

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
            ambient_occlusion_map: Texture::from_file(
                renderer, "./assets/texture_ambientOcclusion.png", TextureFormat::LinearRed
            )?,
        };

        let last_viewport = Viewport::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0));

        Ok(ViewportView {
            render_world,
            material,
            last_viewport,
            events: editor.subscribe(),

            move_button_started_over_ui: false,
            camera_position: Vector3::new(0.0, 2.0, 5.0),
            camera_pitch: 0.0,
            camera_yaw: 0.0,
        })
    }

    pub fn update<W: AdvancedWindow>(
        &mut self, delta: f32, editor: &mut MapEditor, input: &InputModel,
        renderer: &R, window: &mut W, log: &Logger,
    ) {
        // Check if we got a select click
        if input.primary_action.pressed {
            self.select_at_cursor(editor, input, log);
        }

        // Check if we got model events
        while let Some(ev) = self.events.try_recv() {
            match ev {
                MapEditorEvent::NewBrush(index) => {
                    self.add_brush(&editor.map().brushes[index], renderer)
                },
            }
        }

        // Update the camera based on input
        self.update_camera(delta, input, window);
    }

    pub fn render(
        &mut self,
        frame: &mut R::Frame,
        renderer: &mut R,
        window_renderer: &mut R::WindowRenderer,
        world3d_renderer: &mut WR,
        world3d_rendertarget: &mut World3DRenderTarget<R, WR>,
    ) {
        // Create a viewport that doesn't overlap the UI
        // TODO: Query viewport height offset from the UI's ribbon size
        let offset = Vector2::new(0.0, 104.0);
        let mut viewport = Viewport::new(
            offset,
            window_renderer.size().cast() - offset,
        );

        // Fix invalid viewports, they won't be visible but at least they won't crash
        if viewport.size.x < 1.0 {
            viewport.size.x = 1.0;
        }
        if viewport.size.y < 1.0 {
            viewport.size.y = 1.0;
        }

        world3d_renderer.render(
            &self.render_world, &self.create_camera(),
            world3d_rendertarget, &self.last_viewport,
            renderer, window_renderer, frame
        );
        self.last_viewport = viewport;
    }

    fn select_at_cursor(&self, editor: &mut MapEditor, input: &InputModel, log: &Logger) {
        // Translate the window coordinates to normalized screen coordintes for the viewport
        // TODO: Add this as a function to viewport
        let viewport_cursor_pixel = input.cursor_pixel_position - self.last_viewport.position;
        let normalized_cursor = Vector2::new(
            (viewport_cursor_pixel.x / self.last_viewport.size.x) * 2.0 - 1.0,
            (viewport_cursor_pixel.y / self.last_viewport.size.y) * 2.0 - 1.0,
        );

        // If these normalized coordinates are out of range, we should have no selection change.
        // This includes de-selecting because this is most likely a UI click.
        if normalized_cursor.x < -1.0 || normalized_cursor.x > 1.0 ||
           normalized_cursor.y < -1.0 || normalized_cursor.y > 1.0 {
            return;
        }

        // Create a ray matching the normalized screen coordinate
        // TODO: Add this as a function to camera
        let matrix = self.create_camera().screen_to_world_matrix(&self.last_viewport);
        // The matrix approaches infinity distance towards 0, 1 is near clipping plane
        let start = matrix.transform_point(Point3::from_vec(normalized_cursor.extend(1.0)));
        let end = matrix.transform_point(Point3::from_vec(normalized_cursor.extend(0.9)));
        let direction = (end - start).normalize();
        let ray = Ray3::new(start, direction);

        // Check all brushes to see if we got ray hits, we need to get the closest one
        let mut closest: Option<(_, f32)> = None;
        for i in 0..editor.map().brushes.len() {
            let brush = &editor.map().brushes[i];
            for face in &brush.faces {
                // TODO: We can skip faces that can't possibly be closer than the point we have,
                // use the nearest vertex to do that.

                if let Some(intersection) = face.check_intersection(ray, brush) {
                    // Check if this one's closer
                    if closest.map(|v| intersection.distance2 < v.1).unwrap_or(true) {
                        closest = Some((i, intersection.distance2));
                    }
                }
            }
        }

        if input.add_to_selection.state != ButtonState::Press {
            editor.deselect_all();
        }

        if let Some((brush_index, _)) = closest {
            info!(log, "Selecting brush {}", brush_index);
            editor.select(brush_index);
        }
    }

    fn update_camera<W: AdvancedWindow>(
        &mut self, delta: f32, input: &InputModel, window: &mut W
    ) {
        if input.camera_move.state == ButtonState::Release {
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
        if input.forward.state == ButtonState::Press { axes.y += 1.0; }
        if input.backward.state == ButtonState::Press { axes.y -= 1.0; }
        if input.left.state == ButtonState::Press { axes.x -= 1.0; }
        if input.right.state == ButtonState::Press { axes.x += 1.0; }
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
        // TODO: Use triangles functions from map
        for face in &brush.faces {
            let normal = face.normal(brush);

            // Fan-triangulage the face
            // TODO: Optionally support concave faces
            // TODO: Use per-triangle normals
            let fan_anchor = brush.vertices[face.indices[0]];
            let mut last_vertex = brush.vertices[face.indices[1]];
            for index in face.indices.iter().skip(2) {
                let vertex = brush.vertices[*index];
                let indices_start = vertices.len() as u32;

                // We estimate UVs based on world coordinates and the plane normal, in future the
                // user should be able to specify these
                let origin = Point3::new(0.0, 0.0, 0.0);
                let plane = Plane::new(normal, 0.0);
                let axes = create_axes_for_plane(&plane);
                let uv_scale = 0.5;

                vertices.push(Vertex {
                    position: fan_anchor,
                    normal,
                    uv: project_3d_to_2d(fan_anchor, axes, origin) * uv_scale,
                });
                vertices.push(Vertex {
                    position: last_vertex,
                    normal,
                    uv: project_3d_to_2d(last_vertex, axes, origin) * uv_scale,
                });
                vertices.push(Vertex {
                    position: vertex,
                    normal,
                    uv: project_3d_to_2d(vertex, axes, origin) * uv_scale,
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

// TODO: This is copied in multiple places to find points on a brush plane, libraryify that functionality
fn create_axes_for_plane(plane: &Plane<f32>) -> (Vector3<f32>, Vector3<f32>) {
    // Figure out if we should use an up vector to get a perpendicular or a X+1, it needs to be not
    // a parallel.
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = Vector3::new(1.0, 1.0, 0.0);
    let perp_seed = if plane.n.y > 0.9 || plane.n.y < -0.9 { right } else { up };

    // Now use that seed vector to create an perpendicular, then use that to create another
    let x_axis = plane.n.cross(perp_seed).normalize();
    let y_axis = plane.n.cross(x_axis).normalize();

    (x_axis, y_axis)
}

// TODO: This is copied in multiple places to find points on a brush plane, libraryify that functionality
fn project_3d_to_2d(
    point: Point3<f32>, axes: (Vector3<f32>, Vector3<f32>), origin: Point3<f32>
) -> Point2<f32> {
    let relative_point = point - origin;

    //let separation = plane.n.dot(intersection_relative);
    let x = axes.0.dot(relative_point);
    let y = axes.1.dot(relative_point);

    Point2::new(x, y)
}
