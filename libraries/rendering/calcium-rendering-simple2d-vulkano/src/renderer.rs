use std::sync::{Arc};
use std::rc::{Rc};
use std::cell::{RefCell};

use cgmath::{Vector2, Matrix4};
use vulkano::sync::{GpuFuture};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::buffer::cpu_pool::{CpuBufferPool, CpuBufferPoolSubbuffer};
use vulkano::memory::pool::{StdMemoryPool};

use calcium_rendering::{Renderer, Error};
use calcium_rendering::texture::{Texture};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer, RenderBatch, ShaderMode, Simple2DRenderPassRaw, Simple2DRenderPass, Projection};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoFrame};
use calcium_rendering_vulkano_shaders::{simple2d_vs, simple2d_fs};

use {VkVertex, VulkanoSimple2DRenderTargetRaw, RenderTargetData};

struct RendererData {
    dummy_texture: Arc<Texture<VulkanoRenderer>>,

    matrix_pool: CpuBufferPool<simple2d_vs::ty::MatrixData>,
    mode_buffers: Vec<Arc<CpuAccessibleBuffer<simple2d_fs::ty::ModeData>>>,
}

pub struct VulkanoSimple2DRenderer {
    data: Rc<RendererData>,
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
            data: Rc::new(RendererData {
                dummy_texture,

                matrix_pool,
                mode_buffers,
            }),
            vs, fs,
        })
    }
}

impl Simple2DRenderer<VulkanoRenderer> for VulkanoSimple2DRenderer {
    type RenderTargetRaw = VulkanoSimple2DRenderTargetRaw;
    type RenderPassRaw = VulkanoSimple2DRenderPassRaw;

    fn start_pass<'a>(
        &self,
        frame: &'a mut VulkanoFrame,
        render_target: &'a mut Simple2DRenderTarget<VulkanoRenderer, Self>,
        renderer: &mut VulkanoRenderer,
    ) -> Simple2DRenderPass<'a, VulkanoRenderer, Self> {
        // Give the renderer an opportunity to insert any commands it had queued up, this is used
        //  to copy textures for example. This always has to be done right before a render pass.
        frame.future = Some(renderer.submit_queued_commands(frame.future.take().unwrap()));

        // Start the command buffer, this will contain the draw commands
        let buffer_builder = {
            let clear_values = render_target.raw.clear_values();
            let framebuffer = render_target.raw.framebuffer_for(frame.image_num, renderer);

            AutoCommandBufferBuilder::new(
                    renderer.device().clone(), renderer.graphics_queue().family()
                ).unwrap()
                .begin_render_pass(framebuffer.clone(), false, clear_values).unwrap()
        };

        Simple2DRenderPass::raw_new(
            VulkanoSimple2DRenderPassRaw {
                buffer_builder: Some(buffer_builder),
                renderer_data: self.data.clone(),
                render_target_data: render_target.raw.data().clone(),
            }, frame
        )
    }

    fn finish_pass<'a>(
        &self, mut pass: Simple2DRenderPass<'a, VulkanoRenderer, Self>, renderer: &mut VulkanoRenderer,
    ) {
        // Finish the command buffer
        let command_buffer = pass.raw_mut().buffer_builder.take().unwrap()
            .end_render_pass().unwrap()
            .build().unwrap();

        // Submit the command buffer
        let future = Box::new(pass.frame_mut().future.take().unwrap()
            .then_execute(renderer.graphics_queue().clone(), command_buffer)
            .unwrap()
        );
        pass.frame_mut().future = Some(future);

        // Make sure the pass doesn't panic
        pass.mark_finished();
    }
}

pub struct VulkanoSimple2DRenderPassRaw {
    buffer_builder: Option<AutoCommandBufferBuilder>,
    renderer_data: Rc<RendererData>,
    render_target_data: Rc<RefCell<RenderTargetData>>,
}

impl Simple2DRenderPassRaw<VulkanoRenderer> for VulkanoSimple2DRenderPassRaw {
    fn render_batches(
        &mut self,
        batches: &[RenderBatch<VulkanoRenderer>], projection: Projection,
        frame: &mut VulkanoFrame,
        renderer: &mut VulkanoRenderer,
    ) {
        // Create a projection matrix that just matches coordinates to pixels
        let proj =
            // OpenGL expectation of clip space is different from Vulkan
            Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0) *
            // The projection matrix, coming from Projection, is in OpenGL format
            projection.to_matrix(frame.size);

        // Create a buffer for the matrix data to be sent over in
        let total_matrix_raw = proj.into();
        let matrix_data_buffer = Arc::new(self.renderer_data.matrix_pool.next(
            simple2d_vs::ty::MatrixData {
                total: total_matrix_raw,
            }
        ));

        // Go over all batches
        let mut buffer_builder = self.buffer_builder.take().unwrap();
        for batch in batches {
            buffer_builder = self.render_batch(
                &batch, buffer_builder,
                frame.size, renderer,
                &matrix_data_buffer,
            );
        }
        self.buffer_builder = Some(buffer_builder);
    }
}

impl VulkanoSimple2DRenderPassRaw {
    fn render_batch(
        &mut self, batch: &RenderBatch<VulkanoRenderer>, builder: AutoCommandBufferBuilder,
        size: Vector2<u32>, renderer: &VulkanoRenderer,
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
                (0, self.renderer_data.dummy_texture.raw.image(),
                    self.renderer_data.dummy_texture.raw.sampler()),
            &ShaderMode::Texture(ref texture) =>
                (1, texture.raw.image(), texture.raw.sampler()),
            &ShaderMode::Mask(ref texture) =>
                (2, texture.raw.image(), texture.raw.sampler()),
        };

        // Get a buffer containing the mode data
        let mode_data_buffer = self.renderer_data.mode_buffers[mode_id].clone();

        // Create the uniform data set to send over
        let set = Arc::new(self.render_target_data.borrow_mut().set_pool_mut().next()
            .add_buffer(matrix_data_buffer.clone()).unwrap()
            .add_sampled_image(image.clone(), sampler.clone()).unwrap()
            .add_buffer(mode_data_buffer).unwrap()
            .build().unwrap()
        );

        // Add the draw command to the command buffer
        builder.draw(
            self.render_target_data.borrow().pipeline().clone(),
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
