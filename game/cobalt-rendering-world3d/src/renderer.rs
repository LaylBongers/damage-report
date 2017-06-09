use slog::{Logger};
use vulkano::command_buffer::{CommandBufferBuilder};
use vulkano::sync::{GpuFuture};

use cobalt_rendering::vulkano_backend::{VulkanoBackend};
use cobalt_rendering::{Target, Frame, Backend};
use geometry_buffer::{GeometryBuffer};
use geometry_renderer::{GeometryRenderer};
use lighting_renderer::{LightingRenderer};
use {Camera, World};

pub struct Renderer {
    geometry_buffer: GeometryBuffer,
    geometry_renderer: GeometryRenderer,
    lighting_renderer: LightingRenderer,
}

impl Renderer {
    pub fn new(log: &Logger, target: &Target<VulkanoBackend>) -> Self {
        info!(log, "Initializing world renderer");

        let geometry_buffer = GeometryBuffer::new(
            log, target, target.backend().swapchain().depth_attachment.clone()
        );
        let geometry_renderer = GeometryRenderer::new(log, target, &geometry_buffer);

        let lighting_renderer = LightingRenderer::new(log, target);

        Renderer {
            geometry_buffer,
            geometry_renderer,
            lighting_renderer,
        }
    }

    pub fn render(
        &mut self, log: &Logger,
        target: &mut Target<VulkanoBackend>, frame: &mut Frame, camera: &Camera, world: &World
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
            log, target, &self.geometry_buffer, camera, world
        ).build().unwrap();
        let lighting_command_buffer = self.lighting_renderer.build_command_buffer(
            target, frame, &self.geometry_buffer, camera, world
        ).build().unwrap();

        // Add the command buffers to the future we're building up, making sure they're in the
        //  right sequence. geometry buffer first, then the lighting pass that depends on the
        //  geometry buffer.
        let future = frame.future.take().unwrap()
            .then_execute(target.backend().graphics_queue().clone(), geometry_command_buffer).unwrap()
            .then_execute(target.backend().graphics_queue().clone(), lighting_command_buffer).unwrap();
        frame.future = Some(Box::new(future));
    }
}
