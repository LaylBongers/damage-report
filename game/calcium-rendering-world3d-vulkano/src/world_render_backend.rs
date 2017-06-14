use slog::{Logger};
use vulkano::command_buffer::{CommandBufferBuilder};
use vulkano::sync::{GpuFuture};

use calcium_rendering::{RenderSystem};
use calcium_rendering_vulkano::{VulkanoFrame, VulkanoRenderBackend};
use calcium_rendering_world3d::{Camera, RenderWorld, WorldRenderBackend};

use geometry_buffer::{GeometryBuffer};
use geometry_renderer::{GeometryRenderer};
use lighting_renderer::{LightingRenderer};
use mesh::{BackendMeshes};

pub struct VulkanoWorldRenderBackend {
    pub geometry_buffer: GeometryBuffer,
    geometry_renderer: GeometryRenderer,
    lighting_renderer: LightingRenderer,
    meshes: BackendMeshes,
}

impl VulkanoWorldRenderBackend {
    pub fn new(log: &Logger, backend: &VulkanoRenderBackend) -> VulkanoWorldRenderBackend {
        info!(log, "Initializing world renderer");

        let geometry_buffer = GeometryBuffer::new(
            log, backend, backend.target_swapchain.depth_attachment.clone()
        );
        let geometry_renderer = GeometryRenderer::new(log, backend, &geometry_buffer);

        let lighting_renderer = LightingRenderer::new(log, backend);

        VulkanoWorldRenderBackend {
            geometry_buffer,
            geometry_renderer,
            lighting_renderer,
            meshes: BackendMeshes::new(),
        }
    }
}

impl WorldRenderBackend for VulkanoWorldRenderBackend {
    type RenderBackend = VulkanoRenderBackend;
    type Frame = VulkanoFrame;

    fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<VulkanoRenderBackend>,
        frame: &mut VulkanoFrame,
        camera: &Camera, world: &RenderWorld
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

        // Build up the command buffers that contain all the rendering commands, telling the driver
        //  to actually render triangles to buffers. No actual rendering is done here, we just
        //  prepare the render passes and drawcalls.
        let geometry_command_buffer = self.geometry_renderer.build_command_buffer(
            log, &mut render_system.backend, &mut self.meshes, &self.geometry_buffer, camera, world
        ).build().unwrap();
        let lighting_command_buffer = self.lighting_renderer.build_command_buffer(
            &mut render_system.backend, frame, &self.geometry_buffer, camera, world
        ).build().unwrap();

        // Add the command buffers to the future we're building up, making sure they're in the
        //  right sequence. geometry buffer first, then the lighting pass that depends on the
        //  geometry buffer.
        let future = frame.future.take().unwrap()
            .then_execute(render_system.backend.graphics_queue.clone(), geometry_command_buffer)
            .unwrap()
            .then_execute(render_system.backend.graphics_queue.clone(), lighting_command_buffer)
            .unwrap();
        frame.future = Some(Box::new(future));
    }
}
