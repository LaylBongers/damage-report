use std::sync::{Arc};

use cgmath::{self, Vector2};
use vulkano::sync::{GpuFuture};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};

use calcium_rendering::{Renderer, Texture, Error, CalciumErrorMappable, WindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer, RenderBatch, ShaderMode, SampleMode};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoFrame, VulkanoWindowRenderer};
use calcium_rendering_vulkano_shaders::{simple2d_vs, simple2d_fs};

use {VkVertex, VulkanoSimple2DRenderTargetRaw};

pub struct VulkanoSimple2DRenderer {
    dummy_texture: Arc<Texture<VulkanoRenderer>>,
    samplers: Samplers,

    pub vs: simple2d_vs::Shader,
    pub fs: simple2d_fs::Shader,
}

impl VulkanoSimple2DRenderer {
    pub fn new(renderer: &mut VulkanoRenderer) -> Result<Self, Error> {
        info!(renderer.log(), "Creating simple2d renderer");
        let dummy_texture = Texture::from_raw_greyscale(
            renderer, &vec![255u8; 8*8], Vector2::new(8, 8)
        )?;

        // Load in the shaders
        debug!(renderer.log(), "Creating simple2d shaders");
        let vs = simple2d_vs::Shader::load(renderer.device().clone()).unwrap();
        let fs = simple2d_fs::Shader::load(renderer.device().clone()).unwrap();

        let samplers = Samplers::new(renderer)?;

        Ok(VulkanoSimple2DRenderer {
            dummy_texture,
            samplers,

            vs, fs,
        })
    }

    fn render_batch(
        &mut self, batch: &RenderBatch<VulkanoRenderer>, builder: AutoCommandBufferBuilder,
        size: Vector2<u32>, renderer: &VulkanoRenderer,
        render_target: &mut Simple2DRenderTarget<VulkanoRenderer, VulkanoSimple2DRenderer>,
        matrix_data_buffer: &Arc<CpuAccessibleBuffer<simple2d_vs::ty::MatrixData>>,
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
                (0, self.dummy_texture.raw.image(), &self.samplers.linear_sampler),
            &ShaderMode::Texture(ref texture, ref sample_mode) =>
                (1, texture.raw.image(), self.samplers.sampler_for_mode(sample_mode)),
            &ShaderMode::Mask(ref texture, ref sample_mode) =>
                (2, texture.raw.image(), self.samplers.sampler_for_mode(sample_mode)),
        };

        // Create a buffer containing the mode data TODO: Avoid re-creating buffers every frame
        let mode_data_buffer = CpuAccessibleBuffer::<simple2d_fs::ty::ModeData>::from_data(
            renderer.device().clone(), BufferUsage::all(),
            simple2d_fs::ty::ModeData { mode: mode_id },
        ).unwrap();

        // Create the uniform data set to send over
        // TODO: Wait for vulkano to add
        let set = Arc::new(
            PersistentDescriptorSet::start(render_target.raw.pipeline().clone(), 0)
                .add_buffer(matrix_data_buffer.clone()).unwrap()
                .add_sampled_image(image.clone(), sampler.clone()).unwrap()
                .add_buffer(mode_data_buffer.clone()).unwrap()
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
        let matrix_data_buffer = CpuAccessibleBuffer::<simple2d_vs::ty::MatrixData>::from_data(
            renderer.device().clone(), BufferUsage::all(),
            simple2d_vs::ty::MatrixData {
                total: total_matrix_raw,
            }
        ).unwrap();

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

struct Samplers {
    linear_sampler: Arc<Sampler>,
    nearest_sampler: Arc<Sampler>,
}

impl Samplers {
    fn new(renderer: &VulkanoRenderer) -> Result<Self, Error> {
        // Set up the samplers for the sampling modes
        let linear_sampler = Sampler::new(
            renderer.device().clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).map_platform_err()?;
        let nearest_sampler = Sampler::new(
            renderer.device().clone(),
            Filter::Nearest,
            Filter::Nearest,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).map_platform_err()?;

        Ok(Samplers {
            linear_sampler,
            nearest_sampler,
        })
    }

    fn sampler_for_mode(&self, sample_mode: &SampleMode) -> &Arc<Sampler> {
        match sample_mode {
            &SampleMode::Linear => &self.linear_sampler,
            &SampleMode::Nearest => &self.nearest_sampler,
        }
    }
}
