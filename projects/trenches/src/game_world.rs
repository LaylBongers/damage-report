use std::sync::{Arc};

use cgmath::{Vector3, Vector2};
use slog::{Logger};

use calcium_rendering::texture::{Texture, TextureFormat};
use calcium_rendering::{BackendTypes, RenderSystem};
use calcium_rendering_world3d::mesh::{Mesh};
use calcium_rendering_world3d::{RenderWorld, Entity, Material, EntityId, WorldBackendTypes};
use calcium_voxel::{PagedVoxels, PagedVoxelsView};

use input::{InputState, FrameInput};
use player::{Player};
use voxel_system::{VoxelSystem, ChunkEntry};

pub struct GameWorld<T: BackendTypes, WT: WorldBackendTypes<T>> {
    pub player: Player,
    voxels: PagedVoxels,
    voxels_view: PagedVoxelsView,
    voxels_material: Material<T>,
    _wt: ::std::marker::PhantomData<WT>,
}

impl<T: BackendTypes, WT: WorldBackendTypes<T>> GameWorld<T, WT> {
    pub fn new(
        log: &Logger, render_system: &mut RenderSystem<T>, world: &mut RenderWorld<T, WT>
    ) -> Self {
        let player = Player::new();

        world.ambient_light = Vector3::new(0.015, 0.015, 0.02);
        world.directional_light = Vector3::new(1.0, 0.9, 0.8);

        // Initialize the voxel map, setting it up for this game's terrain generation
        let voxels = PagedVoxels::new();

        // Initialize the voxel map view, which creates visible meshes around the player
        let voxels_view = PagedVoxelsView::new();

        let voxels_material = Material {
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
            voxels,
            voxels_view,
            voxels_material,
            _wt: Default::default(),
        }
    }

    pub fn update(
        &mut self, log: &Logger, time: f32,
        render_world: &mut RenderWorld<T, WT>,
        input_state: &InputState, frame_input: &FrameInput
    ) {
        // Update the player based on the input we got so far
        self.player.update(&input_state, &frame_input, time);

        // Update which voxel chunks are active around the player
        let top_player_pos = Vector2::new(self.player.position.x, self.player.position.z);
        self.voxels_view.update(&mut self.voxels);
        //self.voxel_system.update(log, top_player_pos, LoaderUnloader {
        //    log: log.clone(), render_world, voxel_material: &self.voxel_material
        //});
    }
}

/*struct LoaderUnloader<'a, T: BackendTypes, WT: WorldBackendTypes<T>> {
    log: Logger,
    render_world: &'a mut RenderWorld<T, WT>,
    voxel_material: &'a Material<T>,
}

impl<'a, T: BackendTypes, WT: WorldBackendTypes<T>>
    ::voxel_system::LoaderUnloader<T, WT> for LoaderUnloader<'a, T, WT> {
    fn load(&mut self, entry: &mut ChunkEntry, offset: Vector2<f32>, mesh: Arc<Mesh<T, WT>>) {
        // If there already is an entity here, remove it
        if let Some(entity) = entry.entity {
            debug!(self.log, "Replacing existing terrain chunk");
            self.render_world.remove_entity(entity);
        }

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
}*/