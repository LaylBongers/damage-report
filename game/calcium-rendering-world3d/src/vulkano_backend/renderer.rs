use std::collections::{HashMap};
use std::sync::{Arc};
use slog::{Logger};
use vulkano::command_buffer::{CommandBufferBuilder};
use vulkano::sync::{GpuFuture};

use calcium_rendering::{Target};
use calcium_rendering_vulkano::{VulkanoTargetBackend, Frame};

use geometry_buffer::{GeometryBuffer};
use geometry_renderer::{GeometryRenderer};
use lighting_renderer::{LightingRenderer};
use vulkano_backend::mesh::{VulkanoMeshBackend};
use {Camera, World, RendererBackend, Mesh};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct MeshId(usize);

pub struct VulkanoRendererBackend {
    pub geometry_buffer: GeometryBuffer,
    geometry_renderer: GeometryRenderer,
    lighting_renderer: LightingRenderer,
    meshes: BackendMeshes,
}

impl VulkanoRendererBackend {
    pub fn new(log: &Logger, target: &Target<VulkanoTargetBackend>) -> Self {
        info!(log, "Initializing world renderer");

        let geometry_buffer = GeometryBuffer::new(
            log, target, target.backend().swapchain().depth_attachment.clone()
        );
        let geometry_renderer = GeometryRenderer::new(log, target, &geometry_buffer);

        let lighting_renderer = LightingRenderer::new(log, target);

        VulkanoRendererBackend {
            geometry_buffer,
            geometry_renderer,
            lighting_renderer,
            meshes: BackendMeshes::new(),
        }
    }
}

impl RendererBackend for VulkanoRendererBackend {
    type TargetBackend = VulkanoTargetBackend;

    fn render(
        &mut self, log: &Logger,
        target: &mut Target<VulkanoTargetBackend>, frame: &mut Frame,
        camera: &Camera, world: &World
    ) {
        // This is a deferred renderer, so what we will do is first build up the "geometry buffer",
        //  which is a framebuffer made up from various images to keep track of the data needed for
        //  lighting for every pixel. Then, we run the lighting pass over the geometry buffer,
        //  meaning we only have to do lighting "per-screen-pixel" rather than "per-triangle-pixel"
        // TODO: A further optimization is using light geometry to only light the pixels relevant
        //  to the light. This involves using additive blending rather than adding it all up in the
        //  shader while looping through all lights.
        // TODO: This can be done with a single render pass with subpasses, right now I've just
        //  implemented it with separate submitted command buffers because I understand it better
        //  than subpasses at the moment.

        let backend = target.backend_mut();

        // Build up the command buffers that contain all the rendering commands, telling the driver
        //  to actually render triangles to buffers. No actual rendering is done here, we just
        //  prepare the render passes and drawcalls.
        let geometry_command_buffer = self.geometry_renderer.build_command_buffer(
            log, backend, &mut self.meshes, &self.geometry_buffer, camera, world
        ).build().unwrap();
        let lighting_command_buffer = self.lighting_renderer.build_command_buffer(
            backend, frame, &self.geometry_buffer, camera, world
        ).build().unwrap();

        // Add the command buffers to the future we're building up, making sure they're in the
        //  right sequence. geometry buffer first, then the lighting pass that depends on the
        //  geometry buffer.
        let future = frame.future.take().unwrap()
            .then_execute(backend.graphics_queue().clone(), geometry_command_buffer).unwrap()
            .then_execute(backend.graphics_queue().clone(), lighting_command_buffer).unwrap();
        frame.future = Some(Box::new(future));
    }
}

pub struct BackendMeshes {
    meshes: HashMap<MeshId, VulkanoMeshBackend>,
}

impl BackendMeshes {
    fn new() -> BackendMeshes {
        BackendMeshes {
            meshes: HashMap::new(),
        }
    }

    // TODO: This is a near-exact copy of the backend texture loading system, make a common helper
    //  structure so the functionality can be rolled into that.
    pub fn request_mesh(
        &mut self, log: &Logger, target_backend: &VulkanoTargetBackend, texture: &Arc<Mesh>
    ) -> Option<&VulkanoMeshBackend> {
        // Look up the texture from the texture backend storage, or add it if it isn't there yet
        let mesh_backend = self.lookup_or_submit_mesh(log, target_backend, texture);

        // Right now it will be immediately ready
        // TODO: Offload mesh loading to a separate thread and check if it's ready
        Some(mesh_backend)
    }

    fn lookup_or_submit_mesh(
        &mut self, log: &Logger, target_backend: &VulkanoTargetBackend, texture: &Arc<Mesh>
    ) -> &VulkanoMeshBackend {
        let key = MeshId(arc_key(&texture));

        // If we don't have this texture yet, submit it first
        if !self.meshes.contains_key(&key) {
            self.submit_mesh(log, target_backend, texture);
        }

        self.meshes.get(&key).unwrap()
    }

    fn submit_mesh(&mut self, log: &Logger, target_backend: &VulkanoTargetBackend, mesh: &Arc<Mesh>) {
        // TODO: Offload loading to a separate thread

        // Start by loading in the actual mesh
        let mesh_backend = VulkanoMeshBackend::from_vertices_indices(
            log, target_backend, &mesh.vertices, &mesh.indices
        );

        // Store the mesh backend, maintaining its ID so we can look it back up
        let texture_id = self.store_mesh(&mesh, mesh_backend);
    }

    fn store_mesh(
        &mut self, texture: &Arc<Mesh>, texture_backend: VulkanoMeshBackend
    ) -> MeshId {
        let key = MeshId(arc_key(texture));

        // First make sure this texture doesn't already exist, this shouldn't ever happen, but it's
        // not that expensive to make sure
        if self.meshes.contains_key(&key) {
            panic!("Mesh backend already exists for mesh")
        }

        // Now that we're sure, we can submit the texture
        self.meshes.insert(key, texture_backend);

        key
    }
}

fn arc_key<T>(value: &Arc<T>) -> usize {
    value.as_ref() as *const T as usize
}
