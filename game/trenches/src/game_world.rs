use cgmath::{Vector2, Vector3};
use slog::{Logger};

use calcium_rendering::{Texture, TextureFormat};
use calcium_rendering_world3d::{RenderWorld, Entity, Material, Mesh, Vertex};

use input::{InputState, FrameInput};
use player::{Player};
use voxel_grid::{VoxelGrid};

pub struct GameWorld {
    pub player: Player,
    _voxels: VoxelGrid,
}

impl GameWorld {
    pub fn new(log: &Logger, world: &mut RenderWorld) -> Self {
        let player = Player::new();

        world.ambient_light = Vector3::new(0.005, 0.005, 0.005);
        world.directional_light = Vector3::new(0.5, 0.5, 0.5);
        world.add_light(::calcium_rendering_world3d::Light {
            position: Vector3::new(0.0, 1.5, 0.0),
            color: Vector3::new(1.0, 1.0, 1.0),
            radius: 10.0,
        });

        // Add a flat floor to have something as reference
        let floor_mesh = Mesh::from_flat_vertices(log, &floor_vertices());
        let floor_material = Material {
            base_color: Texture::new(
                "./assets/texture_normal.png", TextureFormat::Srgb
            ),
            normal_map: Texture::new(
                "./assets/texture_normal.png", TextureFormat::Linear
            ),
            metallic_map: Texture::new(
                "./assets/texture_metallic.png", TextureFormat::LinearRed
            ),
            roughness_map: Texture::new(
                "./assets/texture_roughness.png", TextureFormat::LinearRed
            ),
        };
        world.add_entity(Entity {
            position: Vector3::new(0.0, 0.0, 0.0),
            mesh: floor_mesh,
            material: floor_material.clone(),
        });

        // Create the in-world voxels
        let mut voxels = VoxelGrid::new(Vector3::new(32, 32, 32));
        for x in 0..voxels.size().x {
            for z in 0..voxels.size().z {
                voxels.set_at(Vector3::new(x, 0, z), true);
            }
        }
        // Remove the positive X 1 voxel just to test
        voxels.set_at(Vector3::new(1, 0, 0), false);

        // Create a mesh from the voxel grid
        if let Some(triangles) = voxels.triangulate() {
            world.add_entity(Entity {
                position: Vector3::new(0.0, 0.0, 0.0),
                mesh: Mesh::from_flat_vertices(log, &triangles),
                material: floor_material,
            });
        }

        GameWorld {
            player,
            _voxels: voxels,
        }
    }

    pub fn update(
        &mut self, time: f32, _world: &mut RenderWorld,
        input_state: &InputState, frame_input: &FrameInput
    ) {
        // Update the player based on the input we got so far
        self.player.update(&input_state, &frame_input, time);
    }
}

fn floor_vertices() -> Vec<Vertex> {
    vec!(
        Vertex {
            position: Vector3::new(-20.0, 0.0, -20.0),
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
