use std::sync::{Arc};

use cgmath::{self, Vector2};
use vulkano::sync::{GpuFuture};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::buffer::cpu_pool::{CpuBufferPool, CpuBufferPoolSubbuffer};
use vulkano::memory::pool::{StdMemoryPool};

use calcium_rendering::{Renderer, Error, WindowRenderer};
use calcium_rendering::texture::{Texture};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer, RenderBatch, ShaderMode};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoFrame, VulkanoWindowRenderer};
use calcium_rendering_vulkano_shaders::{simple2d_vs, simple2d_fs};

use {VkVertex, VulkanoSimple2DRenderTargetRaw};

pub struct VulkanoSimple2DRenderer {
    dummy_texture: Arc<Texture<VulkanoRenderer>>,

    matrix_pool: CpuBufferPool<simple2d_vs::ty::MatrixData>,
    mode_buffers: Vec<Arc<CpuAccessibleBuffer<simple2d_fs::ty::ModeData>>>,

    pub vs: simple2d_vs::Shader,
    pub fs: simple2d_fs::Shader,
}

impl VulkanoSimple2DRenderer {
    pub fn new(renderer: &mut VulkanoRenderer) -> Result<Self, Error> {
        info!(renderer.log(), "Creating simple2d renderer");
        let dummy_texture = Texture::new()
            .from_bytes(vec![255u8; 8*8], Vector2::new(8, 8), false)
            .as_single_channel()
            .build(renderer)?;

        // Load in the shaders
        debug!(renderer.log(), "Creating simple2d shaders");
        let vs = simple2d_vs::Shader::load(renderer.device().clone()).unwrap();
        let fs = simple2d_fs::Shader::load(renderer.device().clone()).unwrap();

        // Set up the CPU buffer pools we'll use to upload various data
        let matrix_pool = CpuBufferPool::new(
            renderer.device().clone(), BufferUsage::uniform_buffer(),
        );

        // Create pre-made mode buffers that can be re-used
        let mut mode_buffers = Vec::new();
        for mode_id in 0..4 {
            let buffer = CpuAccessibleBuffer::from_data(
                renderer.device().clone(), BufferUsage::uniform_buffer(),
                simple2d_fs::ty::ModeData { mode: mode_id }
            ).unwrap();
            mode_buffers.push(buffer);
        }

        Ok(VulkanoSimple2DRenderer {
            dummy_texture,

            matrix_pool,
            mode_buffers,

            vs, fs,
        })
    }

    fn render_batch(
        &mut self, batch: &RenderBatch<VulkanoRenderer>, builder: AutoCommandBufferBuilder,
        size: Vector2<u32>, renderer: &VulkanoRenderer,
        render_target: &mut Simple2DRenderTarget<VulkanoRenderer, VulkanoSimple2DRenderer>,
        matrix_data_buffer: &Arc<CpuBufferPoolSubbuffer<simple2d_vs::ty::MatrixData, Arc<StdMemoryPool>>>,
    ) -> AutoCommandBufferBuilder {
        // Create a big mesh of all the rectangles we got told to draw this batch
        let mut vertices = Vec::new();
        for vertex in &batch.vertices {
            vertices.push(VkVertex {
                v_position: vertex.position.into(),
                v_uv: vertex.uv.into(),
                v_color: vertex.color.into(),
            });
        }

        // Create the final vertex buffer that we'll send over to the GPU for rendering
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            renderer.device().clone(), BufferUsage::all(), vertices.into_iter()
        ).unwrap();

        // Get the mode ID this batch has and a texture to render
        // TODO: Figure out a way to avoid having to have a dummy texture
        // TODO: Make use of the sample mode
        let (mode_id, image, sampler) = match &batch.mode {
            &ShaderMode::Color =>
                (0, self.dummy_texture.raw.image(), self.dummy_texture.raw.sampler()),
            &ShaderMode::Texture(ref texture) =>
                (1, texture.raw.image(), texture.raw.sampler()),
            &ShaderMode::Mask(ref texture) =>
                (2, texture.raw.image(), texture.raw.sampler()),
        };

        // Get a buffer containing the mode data
        let mode_data_buffer = self.mode_buffers[mode_id].clone();

        // Create the uniform data set to send over
        let set = Arc::new(render_target.raw.set_pool_mut().next()
            .add_buffer(matrix_data_buffer.clone()).unwrap()
            .add_sampled_image(image.clone(), sampler.clone()).unwrap()
            .add_buffer(mode_data_buffer).unwrap()
            .build().unwrap()
        );

        // Add the draw command to the command buffer
        builder.draw(
            render_target.raw.pipeline().clone(),
            // TODO: When a lot is being rendered, check the performance impact of doing
            //  this here instead of in the pipeline.
            DynamicState {
                viewports: Some(vec!(Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0 .. 1.0,
                    dimensions: [
                        size.x as f32,
                        size.y as f32
                    ],
                })),
                .. DynamicState::none()
            },
            vec!(vertex_buffer.clone()),
            set, ()
        ).unwrap()
    }
}

impl Simple2DRenderer<VulkanoRenderer> for VulkanoSimple2DRenderer {
    type RenderTargetRaw = VulkanoSimple2DRenderTargetRaw;

    fn render(
        &mut self,
        batches: &[RenderBatch<VulkanoRenderer>],
        render_target: &mut Simple2DRenderTarget<VulkanoRenderer, VulkanoSimple2DRenderer>,
        renderer: &mut VulkanoRenderer, window_renderer: &mut VulkanoWindowRenderer,
        frame: &mut VulkanoFrame,
    ) {
        // Give the renderer an opportunity to insert any commands it had queued up, this is used
        //  to copy textures for example. This always has to be done right before a render pass.
        let mut future = renderer.submit_queued_commands(frame.future.take().unwrap());

        // Create a projection matrix that just matches coordinates to pixels
        let size = window_renderer.size();
        let proj = cgmath::ortho(
            0.0, size.x as f32,
            0.0, size.y as f32, // Top/Bottom flipped, cgmath expects a different clip space
            1.0, -1.0
        );

        // Create a buffer for the matrix data to be sent over in
        let total_matrix_raw = proj.into();
        let matrix_data_buffer = Arc::new(self.matrix_pool.next(
            simple2d_vs::ty::MatrixData {
                total: total_matrix_raw,
            }
        ));

        // Start the command buffer, this will contain the draw commands
        let mut command_buffer_builder = {
            let clear_values = render_target.raw.clear_values();
            let framebuffer = render_target.raw.framebuffer_for(frame.image_num, window_renderer);

            AutoCommandBufferBuilder::new(
                    renderer.device().clone(), renderer.graphics_queue().family()
                ).unwrap()
                .begin_render_pass(framebuffer.clone(), false, clear_values).unwrap()
        };

        // Go over all batches
        for batch in batches {
            command_buffer_builder = self.render_batch(
                &batch, command_buffer_builder,
                size, renderer,
                render_target,
                &matrix_data_buffer,
            );
        }

        // Finish the command buffer
        let command_buffer = command_buffer_builder
            .end_render_pass().unwrap()
            .build().unwrap();

        // Submit the command buffer
        future = Box::new(future
            .then_execute(renderer.graphics_queue().clone(), command_buffer)
            .unwrap()
        );
        frame.future = Some(future);
    }
}
