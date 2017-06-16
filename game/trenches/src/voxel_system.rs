use std::thread::{self, JoinHandle};
use std::sync::{Arc};
use std::sync::mpsc::{self, Sender, Receiver};

use cgmath::{Vector3, Vector2, MetricSpace};
use slog::{Logger};
use noise::{Fbm, Point2, NoiseModule};
use num::{clamp};

use calcium_rendering_world3d::mesh::{self, Mesh};
use calcium_rendering_world3d::{EntityId};

use voxel_grid::{VoxelGrid};

pub struct VoxelSystem {
    chunks: Vec<ChunkEntry>,
    _chunk_load_thread: JoinHandle<()>,
    chunk_load_sender: Sender<ClrCommand>,
    chunk_load_return_receiver: Receiver<(Vector2<i32>, Arc<Mesh>)>,
}

impl VoxelSystem {
    pub fn new(log: &Logger) -> Self {
        // Spawn the chunk loading thread
        let (chunk_load_sender, chunk_load_receiver) = mpsc::channel();
        let (chunk_load_return_sender, chunk_load_return_receiver) = mpsc::channel();
        let thread_log = log.clone();
        let _chunk_load_thread = thread::Builder::new()
            .spawn(move || {
                chunk_load_runtime(thread_log, chunk_load_receiver, chunk_load_return_sender);
            }).unwrap();

        VoxelSystem {
            chunks: Vec::new(),
            _chunk_load_thread,
            chunk_load_sender,
            chunk_load_return_receiver,
        }
    }

    pub fn update<L: LoaderUnloader>(
        &mut self, _log: &Logger,
        top_player_pos: Vector2<f32>,
        mut loader: L
    ) {
        let view_radius = 100.0;
        // This is larger to avoid constant loading/unloading
        let unload_radius = view_radius + 50.0;
        let chunk_size: i32 = 32;

        let view_raidus2 = view_radius * view_radius;
        let unload_radius2 = unload_radius * unload_radius;

        // Calculate the AABB bounds of the chunks that should be visible. This is not yet the
        //  final check, as that gets checked more granular using a distance check later on.
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

                // It's not in the list yet, submit this chunk for loading and add it
                self.chunk_load_sender.send(ClrCommand::Load(chunk)).unwrap();
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
                assert!(chunk.entity.is_none());
                let offset = chunk.chunk.cast() * 32.0;

                loader.load(chunk, offset, value.1);
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
                    loader.unload(entity);
                }
            }

            retain
        });
    }
}

impl Drop for VoxelSystem {
    fn drop(&mut self) {
        self.chunk_load_sender.send(ClrCommand::Stop).unwrap();
    }
}

pub struct ChunkEntry {
    pub chunk: Vector2<i32>,
    pub entity: Option<EntityId>,
}

enum ClrCommand {
    Load(Vector2<i32>),
    Stop
}

fn chunk_load_runtime(
    log: Logger, receiver: Receiver<ClrCommand>, sender: Sender<(Vector2<i32>, Arc<Mesh>)>
) {
    let chunk_size = 32;
    let noise = Fbm::new();

    for command in receiver {
        match command {
            ClrCommand::Load(chunk) => {
                // We don't have a chunk for this yet, generate and add it
                info!(log, "Generating terrain chunk {:?}", chunk);

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

pub trait LoaderUnloader {
    fn load(&mut self, entry: &mut ChunkEntry, offset: Vector2<f32>, mesh: Arc<Mesh>);
    fn unload(&mut self, entity_id: EntityId);
}
