use slog::{Logger};
use vulkano::command_buffer::{CommandBufferBuilder};
use vulkano::sync::{GpuFuture};

use cobalt_rendering::{Target, Frame};
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
    pub fn new(log: &Logger, target: &Target) -> Self {
        info!(log, "Initializing world renderer");

        let geometry_buffer = GeometryBuffer::new(log, target);
        let geometry_renderer = GeometryRenderer::new(log, target, &geometry_buffer);

        let lighting_renderer = LightingRenderer::new(log, target);

        Renderer {
            geometry_buffer,
            geometry_renderer,
            lighting_renderer,
        }
    }

    pub fn render(
        &mut self, target: &mut Target, frame: &mut Frame, camera: &Camera, world: &World
    ) {
        // TODO: This can be done with a single render pass with 3 subpasses, right now I've just
        //  implemented it with separate submitted command buffers way because I understand it
        //  better than subpasses at the moment

        // Build up the command buffers that contain all the rendering commands, telling the driver
        //  to actually render triangles to buffers. This is most likely the heaviest part of
        //  rendering.
        let geometry_command_buffer = self.geometry_renderer.build_command_buffer(
            target, &self.geometry_buffer, camera, world
        ).build().unwrap();
        let lighting_command_buffer = self.lighting_renderer.build_command_buffer(
            target, frame, &self.geometry_buffer, camera, world
        ).build().unwrap();

        // Add the command buffers to the future we're building up, making sure they're in the
        //  right sequence. G-buffer first, then the lighting pass that depends on the g-buffer.
        let future = frame.future.take().unwrap()
            .then_execute(target.graphics_queue().clone(), geometry_command_buffer).unwrap()
            .then_execute(target.graphics_queue().clone(), lighting_command_buffer).unwrap();
        frame.future = Some(Box::new(future));
    }
}
