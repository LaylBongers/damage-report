use std::thread::{self, JoinHandle};
use std::sync::{Arc};
use std::sync::mpsc::{self, Sender, Receiver};

use cgmath::{Vector3, Vector2, MetricSpace};
use slog::{Logger};
use noise::{Fbm, Point2, NoiseModule, Turbulence, Exponent};
use num::{clamp};

use calcium_rendering::{Texture, TextureFormat};
use calcium_rendering_world3d::mesh::{self, Mesh};
use calcium_rendering_world3d::{RenderWorld, Entity, Material, EntityId};

use input::{InputState, FrameInput};
use player::{Player};
use voxel_grid::{VoxelGrid};

struct ChunkEntry {
    pub chunk: Vector2<i32>,
    pub entity: Option<EntityId>,
}

pub struct GameWorld {
    pub player: Player,
    voxel_material: Material,
    chunks: Vec<ChunkEntry>,
    _chunk_load_thread: JoinHandle<()>,
    chunk_load_sender: Sender<ClrCommand>,
    chunk_load_return_receiver: Receiver<(Vector2<i32>, Arc<Mesh>)>,
}

impl GameWorld {
    pub fn new(_log: &Logger, world: &mut RenderWorld) -> Self {
        let player = Player::new();

        world.ambient_light = Vector3::new(0.015, 0.015, 0.02);
        world.directional_light = Vector3::new(0.8, 0.75, 0.7);

        let voxel_material = Material {
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

        // Spawn the chunk loading thread
        let (chunk_load_sender, chunk_load_receiver) = mpsc::channel();
        let (chunk_load_return_sender, chunk_load_return_receiver) = mpsc::channel();
        let _chunk_load_thread = thread::Builder::new()
            .spawn(move || {
                chunk_load_runtime(chunk_load_receiver, chunk_load_return_sender);
            }).unwrap();

        GameWorld {
            player,
            voxel_material,
            chunks: Vec::new(),
            _chunk_load_thread,
            chunk_load_sender,
            chunk_load_return_receiver,
        }
    }

    pub fn update(
        &mut self, _log: &Logger, time: f32, render_world: &mut RenderWorld,
        input_state: &InputState, frame_input: &FrameInput
    ) {
        let view_radius = 100.0;
        // This is larger to avoid constant loading/unloading
        let unload_radius = view_radius + 50.0;
        let chunk_size: i32 = 32;

        let view_raidus2 = view_radius * view_radius;
        let unload_radius2 = unload_radius * unload_radius;

        // Update the player based on the input we got so far
        self.player.update(&input_state, &frame_input, time);

        // Calculate the AABB bounds of the chunks that should be visible. This is not yet the
        //  final check, as that gets checked more granular using a distance check later on.
        let top_player_pos = Vector2::new(self.player.position.x, self.player.position.z);
        let min_x = top_player_pos.x - view_radius;
        let min_y = top_player_pos.y - view_radius;
        let min_chunk_x = f32::floor(min_x / chunk_size as f32) as i32;
        let min_chunk_y = f32::floor(min_y / chunk_size as f32) as i32;
        let max_x = top_player_pos.x + view_radius;
        let max_y = top_player_pos.y + view_radius;
        let max_chunk_x = f32::ceil(max_x / chunk_size as f32) as i32;
        let max_chunk_y = f32::ceil(max_y / chunk_size as f32) as i32;

        // Now go over all of those chunks
        for chunk_x in min_chunk_x..max_chunk_x {
            for chunk_y in min_chunk_y..max_chunk_y {
                let chunk = Vector2::new(chunk_x, chunk_y);

                // Check if this chunk's center is in fact within the distance
                let center_pos = (chunk.cast() + Vector2::new(0.5, 0.5)) * chunk_size as f32;
                if top_player_pos.distance2(center_pos) > view_raidus2 {
                    continue;
                }

                // This chunk should be visible, so make sure it's in the list
                if self.chunks.iter().any(|v| v.chunk == chunk) {
                    continue;
                }

                // Submit this chunk for loading
                self.chunk_load_sender.send(ClrCommand::Load(chunk)).unwrap();

                // Remember the chunk
                self.chunks.push(ChunkEntry {
                    chunk,
                    entity: None,
                });
            }
        }

        // Check if we got back any meshes from the chunk generation backend
        while let Ok(value) = self.chunk_load_return_receiver.try_recv() {
            // Try to find a chunk for this returned mesh
            if let Some(ref mut chunk) = self.chunks.iter_mut().find(|c| c.chunk == value.0) {
                let offset = chunk.chunk.cast() * 32.0;

                // Add the mesh to an entity in the world
                let entity = render_world.add_entity(Entity {
                    position: Vector3::new(offset.x, 0.0, offset.y),
                    mesh: value.1,
                    material: self.voxel_material.clone(),
                });
                chunk.entity = Some(entity);
            }

            // If we couldn't find a stored chunk, it was probably removed before we got a mesh
            //  back from the load thread. TODO: Avoid loading in already removed chunks
        }

        // Eliminate any chunks too far away
        self.chunks.retain(|c| {
            let center_pos = (c.chunk.cast() + Vector2::new(0.5, 0.5)) * chunk_size as f32;
            let retain = top_player_pos.distance2(center_pos) <= unload_radius2;

            // If this is going to be removed, remove it from the world
            if !retain {
                if let Some(entity) = c.entity {
                    // TODO: The mesh is not cleaned up yet, implement this in calcium
                    render_world.remove_entity(entity);
                }
            }

            retain
        });
    }
}

impl Drop for GameWorld {
    fn drop(&mut self) {
        self.chunk_load_sender.send(ClrCommand::Stop).unwrap();
    }
}

fn chunk_load_runtime(receiver: Receiver<ClrCommand>, sender: Sender<(Vector2<i32>, Arc<Mesh>)>) {
    let chunk_size = 32;
    let noise = Turbulence::new(Exponent::new(Fbm::new()));

    for command in receiver {
        match command {
            ClrCommand::Load(chunk) => {
                // We don't have a chunk for this yet, generate and add it
                //info!(log, "Generating terrain chunk ({}, {})", chunk.x, chunk.y);

                let offset = chunk * chunk_size;
                let grid = generate_chunk(offset, &noise);

                // Triangulate this voxel grid
                if let Some(triangles) = grid.triangulate() {
                    let (vertices, indices) = mesh::flat_vertices_to_indexed(&triangles);

                    // Create a mesh from the triangle data
                    let mesh = Mesh::new(vertices, indices);

                    // Return the mesh so the main thread can add it
                    sender.send((chunk, mesh)).unwrap();
                }
            },
            ClrCommand::Stop => return,
        }
    }
}

enum ClrCommand {
    Load(Vector2<i32>),
    Stop
}

fn generate_chunk<N: NoiseModule<Point2<f32>, Output=f32>>(
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
