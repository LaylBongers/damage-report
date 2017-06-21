use std::sync::{Arc};

use cgmath::{Vector3, Vector2};
use slog::{Logger};

use calcium_rendering::{BackendTypes, Texture, TextureFormat, RenderSystem};
use calcium_rendering_world3d::mesh::{Mesh};
use calcium_rendering_world3d::{RenderWorld, Entity, Material, EntityId};

use input::{InputState, FrameInput};
use player::{Player};
use voxel_system::{VoxelSystem, ChunkEntry};

pub struct GameWorld<T: BackendTypes> {
    pub player: Player,
    voxel_system: VoxelSystem,
    voxel_material: Material<T>,
}

impl<T: BackendTypes> GameWorld<T> {
    pub fn new(
        log: &Logger, render_system: &mut RenderSystem<T>, world: &mut RenderWorld<T>
    ) -> Self {
        let player = Player::new();

        world.ambient_light = Vector3::new(0.015, 0.015, 0.02);
        world.directional_light = Vector3::new(1.0, 0.9, 0.8);

        let voxel_material = Material {
            base_color: Texture::new(
                log, render_system, "./assets/texture_base_color.png", TextureFormat::Srgb
            ),
            normal_map: Texture::new(
                log, render_system, "./assets/texture_normal.png", TextureFormat::Linear
            ),
            metallic_map: Texture::new(
                log, render_system, "./assets/texture_metallic.png", TextureFormat::LinearRed
            ),
            roughness_map: Texture::new(
                log, render_system, "./assets/texture_roughness.png", TextureFormat::LinearRed
            ),
        };

        GameWorld {
            player,
            voxel_system: VoxelSystem::new(log),
            voxel_material,
        }
    }

    pub fn update(
        &mut self, log: &Logger, time: f32,
        render_world: &mut RenderWorld<T>,
        input_state: &InputState, frame_input: &FrameInput
    ) {
        // Update the player based on the input we got so far
        self.player.update(&input_state, &frame_input, time);

        // Update which voxel chunks are active around the player
        let top_player_pos = Vector2::new(self.player.position.x, self.player.position.z);
        self.voxel_system.update(log, top_player_pos, LoaderUnloader {
            render_world, voxel_material: &self.voxel_material
        });
    }
}

struct LoaderUnloader<'a, T: BackendTypes> {
    render_world: &'a mut RenderWorld<T>,
    voxel_material: &'a Material<T>,
}

impl<'a, T: BackendTypes> ::voxel_system::LoaderUnloader for LoaderUnloader<'a, T> {
    fn load(&mut self, entry: &mut ChunkEntry, offset: Vector2<f32>, mesh: Arc<Mesh>) {
        // Add the mesh to an entity in the world
        let entity = self.render_world.add_entity(Entity {
            position: Vector3::new(offset.x, 0.0, offset.y),
            mesh: mesh,
            material: (*self.voxel_material).clone(),
        });
        entry.entity = Some(entity);
    }

    fn unload(&mut self, entity_id: EntityId) {
        // TODO: The mesh is not cleaned up yet, implement this in calcium
        self.render_world.remove_entity(entity_id);
    }
}
