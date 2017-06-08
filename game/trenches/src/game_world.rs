use cgmath::{Vector2, Vector3};
use slog::{Logger};
use cobalt_rendering::{Target, Texture, TextureFormat};
use cobalt_rendering_world3d::{World, Entity, Material, Mesh, Vertex};

use input::{InputState, FrameInput};
use player::{Player};

pub struct GameWorld {
    pub player: Player,
}

impl GameWorld {
    pub fn new(log: &Logger, target: &mut Target, world: &mut World) -> Self {
        let player = Player::new();
        world.set_ambient_light(Vector3::new(0.005, 0.005, 0.005));

        // Add a flat floor to have something as reference
        let floor_mesh = Mesh::from_flat_vertices(log, target, &floor_vertices());
        let floor_material = Material {
            base_color: Texture::load(
                log, target, "./assets/texture_normal.png", TextureFormat::Srgb
            ),
            normal_map: Texture::load(
                log, target, "./assets/texture_normal.png", TextureFormat::Linear
            ),
            metallic_map: Texture::load(
                log, target, "./assets/texture_metallic.png", TextureFormat::LinearRed
            ),
            roughness_map: Texture::load(
                log, target, "./assets/texture_roughness.png", TextureFormat::LinearRed
            ),
        };
        world.add_entity(Entity {
            position: Vector3::new(0.0, 0.0, 0.0),
            mesh: floor_mesh,
            material: floor_material,
        });

        GameWorld {
            player,
        }
    }

    pub fn update(
        &mut self, time: f32, _world: &mut World,
        input_state: &InputState, frame_input: &FrameInput
    ) {
        // Update the player based on the input we got so far
        self.player.update(&input_state, &frame_input, time);
    }
}

fn floor_vertices() -> Vec<Vertex> {
    vec!(
        Vertex {
            position: Vector3::new(-10.0, 0.0, -10.0),
            uv: Vector2::new(0.0, 0.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vector3::new(-20.0, 0.0, 20.0),
            uv: Vector2::new(0.0, 1.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vector3::new(20.0, 0.0, -20.0),
            uv: Vector2::new(1.0, 0.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vector3::new(20.0, 0.0, 20.0),
            uv: Vector2::new(1.0, 1.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vector3::new(20.0, 0.0, -20.0),
            uv: Vector2::new(1.0, 0.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vector3::new(-20.0, 0.0, 20.0),
            uv: Vector2::new(0.0, 1.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
    )
}