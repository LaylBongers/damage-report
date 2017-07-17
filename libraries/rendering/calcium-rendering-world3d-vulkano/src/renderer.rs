use vulkano::sync::{GpuFuture};

use calcium_rendering::{Error, Renderer};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoTypes, VulkanoWindowRenderer, VulkanoFrame};
use calcium_rendering_world3d::{World3DRenderer, RenderWorld, Camera, World3DRenderTarget};

use geometry_renderer::{GeometryRenderer};
use lighting_renderer::{LightingRenderer};
use {VulkanoWorld3DTypes};

pub struct VulkanoWorld3DRenderer {
    geometry_renderer: GeometryRenderer,
    lighting_renderer: LightingRenderer,
}

impl VulkanoWorld3DRenderer {
    pub fn new(
        renderer: &VulkanoRenderer,
    ) -> Result<Self, Error> {
        info!(renderer.log(), "Initializing world renderer");

        let geometry_renderer = GeometryRenderer::new(renderer)?;
        let lighting_renderer = LightingRenderer::new(renderer);

        Ok(VulkanoWorld3DRenderer {
            geometry_renderer,
            lighting_renderer,
        })
    }
}

impl World3DRenderer<VulkanoTypes, VulkanoWorld3DTypes> for VulkanoWorld3DRenderer {
    fn render(
        &mut self,
        world: &RenderWorld<VulkanoTypes, VulkanoWorld3DTypes>, camera: &Camera,
        world3d_rendertarget: &mut World3DRenderTarget<VulkanoTypes, VulkanoWorld3DTypes>,
        renderer: &mut VulkanoRenderer, window_renderer: &mut VulkanoWindowRenderer,
        frame: &mut VulkanoFrame,
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
            world, camera, world3d_rendertarget, renderer, window_renderer,
        ).build().unwrap();
        let lighting_command_buffer = self.lighting_renderer.build_command_buffer(
            world, camera, world3d_rendertarget, renderer, window_renderer, frame,
        ).build().unwrap();

        // Add the command buffers to the future we're building up, making sure they're in the
        //  right sequence. geometry buffer first, then the lighting pass that depends on the
        //  geometry buffer.
        let future = frame.future.take().unwrap()
            .then_execute(renderer.graphics_queue().clone(), geometry_command_buffer)
            .unwrap()
            .then_execute(renderer.graphics_queue().clone(), lighting_command_buffer)
            .unwrap();
        frame.future = Some(Box::new(future));
    }
}
