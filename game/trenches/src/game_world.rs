use cgmath::{Vector3, Vector2};
use rayon::prelude::*;
use slog::{Logger};
use noise::{Fbm, Point2, NoiseModule, Turbulence, Exponent};
use num::{clamp};

use calcium_rendering::{Texture, TextureFormat};
use calcium_rendering_world3d::mesh::{self, Mesh, Vertex};
use calcium_rendering_world3d::{RenderWorld, Entity, Material};

use input::{InputState, FrameInput};
use player::{Player};
use voxel_grid::{VoxelGrid};

pub struct GameWorld {
    pub player: Player,
}

impl GameWorld {
    pub fn new(log: &Logger, world: &mut RenderWorld) -> Self {
        let player = Player::new();

        world.ambient_light = Vector3::new(0.015, 0.015, 0.02);
        world.directional_light = Vector3::new(0.8, 0.75, 0.7);

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

        // Create the in-world voxels
        let noise = Turbulence::new(Exponent::new(Fbm::new()));
        struct ParEntry {
            chunk: Vector2<i32>,
            vert_ind: Option<(Vec<Vertex>, Vec<u16>)>,
        }
        let size = 16;
        let half_size = size/2;
        let mut parallel_data: Vec<_> = (0..size*size)
            .map(|v| (v/size - half_size, v%size - half_size))
            .map(|v| ParEntry { chunk: Vector2::new(v.0, v.1), vert_ind: None })
            .collect();
        parallel_data
            .par_iter_mut()
            .for_each(|v| {
                debug!(log, "Generating terrain chunk ({}, {})", v.chunk.x, v.chunk.y);

                let offset = v.chunk * 32;
                let voxels = generate_voxels(offset, &noise);

                // Triangulate this voxel grid
                if let Some(triangles) = voxels.triangulate() {
                    v.vert_ind = Some(mesh::flat_vertices_to_indexed(&triangles));
                }
            });
        for v in parallel_data {
            if let Some(vert_ind) = v.vert_ind {
                let offset = v.chunk * 32;

                // Finally add the triangles to the world
                world.add_entity(Entity {
                    position: Vector3::new(offset.x, 0, offset.y).cast(),
                    mesh: Mesh::new(vert_ind.0, vert_ind.1),
                    material: floor_material.clone(),
                });
            }
        }

        GameWorld {
            player,
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

fn generate_voxels<N: NoiseModule<Point2<f32>, Output=f32>>(
    offset: Vector2<i32>, noise: &N
) -> VoxelGrid {
    let mut voxels = VoxelGrid::new(Vector3::new(32, 128, 32));

    // Terrain gen parameters
    let noise_scale = 0.0025;
    let height = 126.0;

    // Generate terrain
    for x in 0..voxels.size().x {
        for z in 0..voxels.size().z {
            let offset_coord = Vector2::new(x, z) + offset;
            let scaled_coord: Point2<f32> = (offset_coord.cast() * noise_scale).into();
            let noise_value: f32 = clamp((noise.get(scaled_coord) + 1.0) * 0.5, 0.0, 1.0);
            let height = (noise_value * height + 1.0) as i32;

            // Actually set the voxels along the height
            for y in 0..height {
                voxels.set_at(Vector3::new(x, y, z), true);
            }
        }
    }

    voxels
}
