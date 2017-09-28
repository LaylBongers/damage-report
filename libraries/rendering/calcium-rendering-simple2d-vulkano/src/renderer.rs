use std::sync::{Arc};

use cgmath::{Vector2, Matrix4};
use vulkano::sync::{GpuFuture};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::buffer::cpu_pool::{CpuBufferPool, CpuBufferPoolSubbuffer};
use vulkano::memory::pool::{StdMemoryPool};

use calcium_rendering::raw::{RawAccess};
use calcium_rendering::texture::{Texture};
use calcium_rendering::{Renderer, Error, Frame};
use calcium_rendering_simple2d::render_data::{RenderBatch, ShaderMode, RenderData, RenderSet};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer};
use calcium_rendering_vulkano::{VulkanoRendererRaw};
use calcium_rendering_vulkano_shaders::{simple2d_vs, simple2d_fs};

use {VkVertex, VulkanoSimple2DRenderTargetRaw};

pub struct VulkanoSimple2DRenderer {
    dummy_texture: Arc<Texture<VulkanoRendererRaw>>,

    matrix_pool: CpuBufferPool<simple2d_vs::ty::MatrixData>,
    mode_buffers: Vec<Arc<CpuAccessibleBuffer<simple2d_fs::ty::ModeData>>>,

    pub vs: simple2d_vs::Shader,
    pub fs: simple2d_fs::Shader,
}

impl VulkanoSimple2DRenderer {
    pub fn new(renderer: &mut Renderer<VulkanoRendererRaw>) -> Result<Self, Error> {
        info!(renderer.log(), "Creating simple2d renderer");
        let dummy_texture = Texture::new()
            .from_bytes(vec![255u8; 8*8], Vector2::new(8, 8), false)
            .as_single_channel()
            .build(renderer)?;

        // Load in the shaders
        debug!(renderer.log(), "Creating simple2d shaders");
        let vs = simple2d_vs::Shader::load(renderer.raw().device().clone()).unwrap();
        let fs = simple2d_fs::Shader::load(renderer.raw().device().clone()).unwrap();

        // Set up the CPU buffer pools we'll use to upload various data
        let matrix_pool = CpuBufferPool::new(
            renderer.raw().device().clone(), BufferUsage::uniform_buffer(),
        );

        // Create pre-made mode buffers that can be re-used
        let mut mode_buffers = Vec::new();
        for mode_id in 0..4 {
            let buffer = CpuAccessibleBuffer::from_data(
                renderer.raw().device().clone(), BufferUsage::uniform_buffer(),
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

    fn render_set(
        &mut self,
        set: &RenderSet<VulkanoRendererRaw>, mut buffer_builder: AutoCommandBufferBuilder,
        frame: &Frame<VulkanoRendererRaw>,
        renderer: &Renderer<VulkanoRendererRaw>,
        render_target: &mut Simple2DRenderTarget<VulkanoRendererRaw, VulkanoSimple2DRenderer>,
    ) -> AutoCommandBufferBuilder {
        // Create a projection matrix that just matches coordinates to pixels
        let proj =
            // OpenGL expectation of clip space is different from Vulkan
            Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0) *
            // The projection matrix, coming from Projection, is in OpenGL format
            set.projection.to_matrix(frame.raw().size);

        // Create a buffer for the matrix data to be sent over in
        let total_matrix_raw = proj.into();
        let matrix_data_buffer = Arc::new(self.matrix_pool.next(
            simple2d_vs::ty::MatrixData {
                total: total_matrix_raw,
            }
        ));

        // Go over all batches
        for batch in &set.batches {
            buffer_builder = self.render_batch(
                &batch, buffer_builder,
                frame.raw().size, renderer, render_target,
                &matrix_data_buffer,
            );
        }

        buffer_builder
    }

    fn render_batch(
        &mut self,
        batch: &RenderBatch<VulkanoRendererRaw>, builder: AutoCommandBufferBuilder,
        size: Vector2<u32>,
        renderer: &Renderer<VulkanoRendererRaw>,
        render_target: &mut Simple2DRenderTarget<VulkanoRendererRaw, VulkanoSimple2DRenderer>,
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
            renderer.raw().device().clone(), BufferUsage::all(), vertices.into_iter()
        ).unwrap();

        // Get the mode ID this batch has and a texture to render
        // TODO: Figure out a way to avoid having to have a dummy texture
        // TODO: Make use of the sample mode
        let (mode_id, image, sampler) = match &batch.mode {
            &ShaderMode::Color =>
                (0, self.dummy_texture.raw().image(),
                    self.dummy_texture.raw().sampler()),
            &ShaderMode::Texture(ref texture) =>
                (1, texture.raw().image(), texture.raw().sampler()),
            &ShaderMode::Mask(ref texture) =>
                (2, texture.raw().image(), texture.raw().sampler()),
        };

        // Get a buffer containing the mode data
        let mode_data_buffer = self.mode_buffers[mode_id].clone();

        // Create the uniform data set to send over
        let set = Arc::new(render_target.raw_mut().set_pool_mut().next()
            .add_buffer(matrix_data_buffer.clone()).unwrap()
            .add_sampled_image(image.clone(), sampler.clone()).unwrap()
            .add_buffer(mode_data_buffer).unwrap()
            .build().unwrap()
        );

        // Add the draw command to the command buffer
        builder.draw(
            render_target.raw().pipeline().clone(),
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

impl Simple2DRenderer<VulkanoRendererRaw> for VulkanoSimple2DRenderer {
    type RenderTargetRaw = VulkanoSimple2DRenderTargetRaw;

    fn render(
        &mut self,
        data: &RenderData<VulkanoRendererRaw>,
        frame: &mut Frame<VulkanoRendererRaw>,
        render_target: &mut Simple2DRenderTarget<VulkanoRendererRaw, Self>,
        renderer: &mut Renderer<VulkanoRendererRaw>,
    ) {
        // Give the renderer an opportunity to insert any commands it had queued up, this is used
        //  to copy textures for example. This always has to be done right before a render pass.
        let future = renderer.raw_mut().submit_queued_commands(frame.raw_mut().future.take().unwrap());

        // Start the command buffer, this will contain the draw commands
        let mut buffer_builder = {
            let clear_values = render_target.raw.clear_values();
            let framebuffer = render_target.raw.framebuffer_for(frame.raw().image_num, renderer);

            AutoCommandBufferBuilder::new(
                    renderer.raw().device().clone(), renderer.raw().graphics_queue().family()
                ).unwrap()
                .begin_render_pass(framebuffer.clone(), false, clear_values).unwrap()
        };

        // Render all render sets
        for set in &data.render_sets {
            buffer_builder = self.render_set(set, buffer_builder, frame, renderer, render_target);
        }

        // Finish the command buffer
        let command_buffer = buffer_builder
            .end_render_pass().unwrap()
            .build().unwrap();

        // Submit the command buffer
        let future = Box::new(future
            .then_execute(renderer.raw().graphics_queue().clone(), command_buffer)
            .unwrap()
        );
        frame.raw_mut().future = Some(future);
    }
}
