use std::sync::{Arc};
use std::iter;

use cgmath::{self, Vector2};
use vulkano::sync::{GpuFuture};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet};
use slog::{Logger};

use calcium_rendering::{Texture};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch, Rectangle, BatchMode};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoWindowRenderer, VulkanoBackendTypes, VulkanoFrame, VulkanoTexture};
use calcium_rendering_vulkano_shaders::{simple2d_vs, simple2d_fs};

use {VkVertex};

pub struct VulkanoSimple2DRenderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    dummy_texture: Arc<VulkanoTexture>,
}

impl VulkanoSimple2DRenderer {
    pub fn new(
        log: &Logger, renderer: &mut VulkanoRenderer, window: &VulkanoWindowRenderer
    ) -> Self {
        info!(log, "Creating simple2d renderer");
        let render_pass = create_render_pass(log, renderer, window);
        let pipeline = create_pipeline(log, renderer, window, render_pass);
        let dummy_texture = VulkanoTexture::from_raw_greyscale(
            log, renderer, &vec![255u8; 8*8], Vector2::new(8, 8)
        );

        VulkanoSimple2DRenderer {
            pipeline,
            dummy_texture,
        }
    }
}

impl Simple2DRenderer<VulkanoBackendTypes> for VulkanoSimple2DRenderer {
    fn render(
        &mut self, renderer: &mut VulkanoRenderer, frame: &mut VulkanoFrame,
        batches: Vec<RenderBatch<VulkanoBackendTypes>>
    ) {
        // Give the renderer an opportunity to insert any commands it had queued up, this is used
        //  to copy textures for example. This always has to be done right before a render pass.
        let mut future = renderer.submit_queued_commands(frame.future.take().unwrap());

        // Create a projection matrix that just matches coordinates to pixels
        let proj = cgmath::ortho(
            0.0, frame.size.x as f32,
            0.0, frame.size.y as f32, // Top/Bottom flipped, cgmath expects a different clip space
            1.0, -1.0
        );

        // Create a buffer for the matrix data to be sent over in
        let total_matrix_raw = proj.into();
        let matrix_data_buffer = CpuAccessibleBuffer::<simple2d_vs::ty::MatrixData>::from_data(
            renderer.device.clone(), BufferUsage::all(),
            Some(renderer.graphics_queue.family()),
            simple2d_vs::ty::MatrixData {
                total: total_matrix_raw,
            }
        ).unwrap();

        // Start the command buffer, this will contain the draw commands
        let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
                renderer.device.clone(), renderer.graphics_queue.family()
            ).unwrap()
            .begin_render_pass(frame.framebuffer.clone(), false, clear_values).unwrap();

        // Go over all batches
        for batch in batches {
            // Create a big mesh of all the rectangles we got told to draw this batch
            let mut vertices = Vec::new();
            for rect in batch.rectangles {
                let start: Vector2<f32> = rect.destination.start.cast();
                let end: Vector2<f32> = rect.destination.end.cast();

                let texture_source = rect.texture_source.unwrap_or(
                    Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0))
                );
                let uv_start: Vector2<f32> = texture_source.start;
                let uv_end: Vector2<f32> = texture_source.end;

                let color = rect.color.into(); // TODO: Convert gamma

                vertices.push(VkVertex {
                    v_position: [start.x, start.y],
                    v_uv: [uv_start.x, uv_start.y],
                    v_color: color,
                });
                vertices.push(VkVertex {
                    v_position: [start.x, end.y],
                    v_uv: [uv_start.x, uv_end.y],
                    v_color: color,
                });
                vertices.push(VkVertex {
                    v_position: [end.x, start.y],
                    v_uv: [uv_end.x, uv_start.y],
                    v_color: color,
                });

                vertices.push(VkVertex {
                    v_position: [end.x, end.y],
                    v_uv: [uv_end.x, uv_end.y],
                    v_color: color,
                });
                vertices.push(VkVertex {
                    v_position: [end.x, start.y],
                    v_uv: [uv_end.x, uv_start.y],
                    v_color: color,
                });
                vertices.push(VkVertex {
                    v_position: [start.x, end.y],
                    v_uv: [uv_start.x, uv_end.y],
                    v_color: color,
                });
            }

            // Create the final vertex buffer that we'll send over to the GPU for rendering
            let vertex_buffer = CpuAccessibleBuffer::from_iter(
                renderer.device.clone(), BufferUsage::all(),
                Some(renderer.graphics_queue.family()),
                vertices.into_iter()
            ).unwrap();

            // Get the mode ID this batch has and a texture to render
            // TODO: Figure out a way to avoid having to have a dummy texture
            let (mode_id, tex_uniform) = match &batch.mode {
                &BatchMode::Color => (0, self.dummy_texture.uniform()),
                &BatchMode::Texture(ref texture) => (1, texture.uniform()),
                &BatchMode::Mask(ref texture) => (2, texture.uniform()),
            };

            // Create a buffer containing the mode data TODO: Avoid re-creating buffers every frame
            let mode_buffer = CpuAccessibleBuffer::<simple2d_fs::ty::ModeData>::from_data(
                renderer.device.clone(), BufferUsage::all(),
                Some(renderer.graphics_queue.family()),
                simple2d_fs::ty::ModeData {
                    mode: mode_id,
                }
            ).unwrap();

            // Create the uniform data set to send over
            // TODO: It's really expensive to constantly create persistent sets, figure out some
            //  way to solve this
            let set = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                .add_buffer(matrix_data_buffer.clone()).unwrap()
                .add_sampled_image(tex_uniform.0, tex_uniform.1).unwrap()
                .add_buffer(mode_buffer.clone()).unwrap()
                .build().unwrap()
            );

            // Add the draw command to the command buffer
            command_buffer_builder = command_buffer_builder
                .draw(
                    self.pipeline.clone(),
                    DynamicState::none(),
                    vec!(vertex_buffer.clone()),
                    set, ()
                ).unwrap()

        }

        // Finish the command buffer
        let command_buffer = command_buffer_builder
            .end_render_pass().unwrap()
            .build().unwrap();

        // Submit the command buffer
        future = Box::new(future
            .then_execute(renderer.graphics_queue.clone(), command_buffer)
            .unwrap()
        );
        frame.future = Some(future);
    }
}

fn create_render_pass(
    log: &Logger, renderer: &VulkanoRenderer, window: &VulkanoWindowRenderer,
) -> Arc<RenderPassAbstract + Send + Sync> {
    debug!(log, "Creating g-buffer render pass");
    #[allow(dead_code)]
    let render_pass = Arc::new(single_pass_renderpass!(renderer.device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: window.swapchain.swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());

    render_pass
}

fn create_pipeline(
    log: &Logger, renderer: &VulkanoRenderer, window: &VulkanoWindowRenderer,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Creating simple2d shaders");
    let vs = simple2d_vs::Shader::load(renderer.device.clone()).unwrap();
    let fs = simple2d_fs::Shader::load(renderer.device.clone()).unwrap();

    // Set up the pipeline itself
    debug!(log, "Creating gbuffer pipeline");
    let dimensions = window.size;
    Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer()
        .triangle_list()
        .viewports(iter::once(Viewport {
            origin: [0.0, 0.0],
            depth_range: 0.0 .. 1.0,
            dimensions: [
                dimensions[0] as f32,
                dimensions[1] as f32
            ],
        }))

        // Which shaders to use
        .vertex_shader(vs.main_entry_point(), ())
        .fragment_shader(fs.main_entry_point(), ())

        .blend_alpha_blending()

        .render_pass(Subpass::from(render_pass, 0).unwrap())
        .build(renderer.device.clone()).unwrap()
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<VkVertex>, _, _>>
}