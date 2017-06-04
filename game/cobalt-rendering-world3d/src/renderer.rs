use std::sync::{Arc};

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4};
use slog::{Logger};
use vulkano::format::{self, Format};
use vulkano::image::{Image};
use vulkano::image::attachment::{AttachmentImage};
use vulkano::format::{ClearValue};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder, DynamicState};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineParams, GraphicsPipelineAbstract};
use vulkano::pipeline::blend::{Blend};
use vulkano::pipeline::depth_stencil::{DepthStencil};
use vulkano::pipeline::input_assembly::{InputAssembly};
use vulkano::pipeline::multisample::{Multisample};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{ViewportsState, Viewport, Scissor};
use vulkano::framebuffer::{Subpass, Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::sync::{GpuFuture};
use cobalt_rendering_shaders::{gbuffer_vs, gbuffer_fs, lighting_vs, lighting_fs};

use cobalt_rendering::{Target, Frame};
use {Camera, World, Entity};

pub struct Renderer {
    gbuffer_framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    gbuffer_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    lighting_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl Renderer {
    pub fn init(log: &Logger, target: &Target) -> Self {
        info!(log, "Initializing world renderer");

        // Create the attachment images that make up the G-buffer
        debug!(log, "Creating g-buffer attachment images");
        let position_attachment = AttachmentImage::new(
            target.device().clone(), target.size().into(), format::R16G16B16A16Sfloat
        ).unwrap();
        let base_color_attachment = AttachmentImage::new(
            target.device().clone(), target.size().into(), format::R8G8B8A8Srgb
        ).unwrap();
        let normal_attachment = AttachmentImage::new(
            target.device().clone(), target.size().into(), format::R8G8B8A8Unorm
        ).unwrap();
        let depth_attachment = AttachmentImage::transient(
            target.device().clone(), target.size().into(), format::D16Unorm
        ).unwrap();

        // Create the deferred render pass
        // TODO: Document better what a render pass does that a framebuffer doesn't
        debug!(log, "Creating g-buffer render pass");
        #[allow(dead_code)]
        let gbuffer_render_pass = Arc::new(single_pass_renderpass!(target.device().clone(),
            attachments: {
                position: {
                    load: Clear,
                    store: Store,
                    format: Format::R16G16B16A16Sfloat,
                    samples: 1,
                },
                base_color: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Srgb,
                    samples: 1,
                },
                normal: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Unorm,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [position, base_color, normal],
                depth_stencil: {depth}
            }
        ).unwrap());

        // Create the off-screen g-buffer framebuffer that we will use to actually tell vulkano
        //  what images we want to render to
        debug!(log, "Creating g-buffer framebuffer");
        let gbuffer_framebuffer = Arc::new(Framebuffer::start(gbuffer_render_pass.clone())
            .add(position_attachment.clone()).unwrap()
            .add(base_color_attachment.clone()).unwrap()
            .add(normal_attachment.clone()).unwrap()
            .add(depth_attachment.clone()).unwrap()
            .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>;

        // Set up the shaders and pipelines
        let gbuffer_pipeline = load_gbuffer_pipeline(log, target, gbuffer_render_pass);
        let lighting_pipeline = load_lighting_pipeline(log, target);

        Renderer {
            gbuffer_framebuffer,
            gbuffer_pipeline,
            lighting_pipeline,
        }
    }

    pub fn render(
        &mut self, target: &mut Target, frame: &mut Frame, camera: &Camera, world: &World
    ) {
        // TODO: This can be done with a single render pass with 3 subpasses, right now I've just
        //  implemented it this way to not stray from the examples

        // Build up the command buffers that contain all the rendering commands, telling the driver
        //  to actually render triangles to buffers. This is most likely the heaviest part of
        //  rendering.
        let deferred_command_buffer = self.build_gbuffer_command_buffer(target, camera, world)
            .build().unwrap();
        let lighting_command_buffer = self.build_lighting_command_buffer(target, frame)
            .build().unwrap();

        // Add the command buffers to the future we're building up, making sure they're in the
        //  right sequence. G-buffer first, then the lighting pass that depends on the g-buffer.
        let future = frame.future.take().unwrap()
            .then_execute(target.graphics_queue().clone(), deferred_command_buffer).unwrap()
            .then_execute(target.graphics_queue().clone(), lighting_command_buffer).unwrap();
        frame.future = Some(Box::new(future));
    }

    pub fn build_gbuffer_command_buffer(
        &mut self, target: &mut Target, camera: &Camera, world: &World,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            target.device().clone(), target.graphics_queue().family()
        ).unwrap();

        let clear_values = vec!(
            // These colors has no special significance, it's just useful for debugging that a lack
            //  of a value is set to black.
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Depth(1.0)
        );
        command_buffer_builder = command_buffer_builder
            .begin_render_pass(self.gbuffer_framebuffer.clone(), false, clear_values).unwrap();

        // Create the projection-view matrix needed for the perspective rendering
        let projection_view = create_projection_view_matrix(target, camera);

        // Go over everything in the world
        for entity in world.entities() {
            command_buffer_builder = self.render_entity(
                entity, target, &projection_view, command_buffer_builder
            );
        }

        // Finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }

    fn render_entity(
        &self,
        entity: &Entity, target: &mut Target,
        projection_view: &Matrix4<f32>,
        command_buffer: AutoCommandBufferBuilder,
    ) -> AutoCommandBufferBuilder {
        // Create a matrix for this world entity
        let model = Matrix4::from_translation(entity.position);
        let total_matrix_raw: [[f32; 4]; 4] = (projection_view * model).into();
        let model_matrix_raw: [[f32; 4]; 4] = model.into();

        // Send the matrices over to the GPU
        let matrix_data_buffer = CpuAccessibleBuffer::<gbuffer_vs::ty::MatrixData>::from_data(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            gbuffer_vs::ty::MatrixData {
                total: total_matrix_raw,
                model: model_matrix_raw,
            }
        ).unwrap();

        // Create the final uniforms set
        let set = Arc::new(simple_descriptor_set!(self.gbuffer_pipeline.clone(), 0, {
            u_matrix_data: matrix_data_buffer,
            u_material_base_color: entity.material.base_color.uniform(),
        }));

        // Perform the actual draw
        command_buffer
            .draw_indexed(
                self.gbuffer_pipeline.clone(), DynamicState::none(),
                vec!(entity.mesh.vertex_buffer.clone()), entity.mesh.index_buffer.clone(),
                set, ()
            ).unwrap()
    }

    pub fn build_lighting_command_buffer(
        &mut self, target: &mut Target, frame: &Frame,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            target.device().clone(), target.graphics_queue().family()
        ).unwrap();

        // Begin by starting the render pass, we're rendering the lighting pass directly to the
        //  final framebuffer for this frame, that framebuffer will be presented to the screen.
        // Because this is the final screen framebuffer all we need to clear is the color and
        //  depth. We still use depth because we may want to do another forward render pass for
        //  transparent objects.
        let clear_values = vec!(
            // This color has no special significance, it's just nicer than pure black
            ClearValue::Float([0.005, 0.005, 0.005, 1.0]),
            ClearValue::Depth(1.0)
        );
        command_buffer_builder = command_buffer_builder
            .begin_render_pass(frame.framebuffer.clone(), false, clear_values).unwrap();

        // Create a buffer for a single screen-sized triangle TODO: Re-use that buffer
        let sst_vertices = vec![
            ScreenSizeTriVertex { v_position: [-1.0, -1.0], v_uv: [0.0, 0.0], },
            ScreenSizeTriVertex { v_position: [-1.0,  3.0], v_uv: [0.0, 2.0], },
            ScreenSizeTriVertex { v_position: [ 3.0, -1.0], v_uv: [2.0, 0.0], },
        ];
        let sst_buffer = CpuAccessibleBuffer::<[ScreenSizeTriVertex]>::from_iter(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            sst_vertices.into_iter()
        ).unwrap();

        // Submit the triangle for rendering
        command_buffer_builder = command_buffer_builder
            .draw(
                self.lighting_pipeline.clone(), DynamicState::none(), vec!(sst_buffer), (), ()
            ).unwrap();

        // Finally, finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }
}

fn load_gbuffer_pipeline(
    log: &Logger, target: &Target,
    gbuffer_render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading gbuffer shaders");
    let vs = gbuffer_vs::Shader::load(target.device()).unwrap();
    let fs = gbuffer_fs::Shader::load(target.device()).unwrap();

    // Set up the pipeline
    debug!(log, "Creating gbuffer pipeline");
    let dimensions = target.images()[0].dimensions().width_height();
    let pipeline_params = GraphicsPipelineParams {
        vertex_input: SingleBufferDefinition::new(),
        vertex_shader: vs.main_entry_point(),
        input_assembly: InputAssembly::triangle_list(),
        tessellation: None,
        geometry_shader: None,
        viewport: ViewportsState::Fixed {
            data: vec![(
                Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0 .. 1.0,
                    dimensions: [
                        dimensions[0] as f32,
                        dimensions[1] as f32
                    ],
                },
                Scissor::irrelevant()
            )],
        },
        raster: Default::default(),
        multisample: Multisample::disabled(),
        fragment_shader: fs.main_entry_point(),
        depth_stencil: DepthStencil::simple_depth_test(),
        blend: Blend::pass_through(),
        render_pass: Subpass::from(gbuffer_render_pass, 0).unwrap(),
    };

    Arc::new(GraphicsPipeline::new(target.device().clone(), pipeline_params).unwrap())
        as Arc<GraphicsPipeline<SingleBufferDefinition<::VkVertex>, _, _>>
}

fn load_lighting_pipeline(
    log: &Logger, target: &Target
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading deferred shaders");
    let vs = lighting_vs::Shader::load(target.device()).unwrap();
    let fs = lighting_fs::Shader::load(target.device()).unwrap();

    // Set up the pipeline
    debug!(log, "Creating deferred pipeline");
    let dimensions = target.images()[0].dimensions().width_height();
    let pipeline_params = GraphicsPipelineParams {
        vertex_input: SingleBufferDefinition::new(),
        vertex_shader: vs.main_entry_point(),
        input_assembly: InputAssembly::triangle_list(),
        tessellation: None,
        geometry_shader: None,
        viewport: ViewportsState::Fixed {
            data: vec![(
                Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0 .. 1.0,
                    dimensions: [
                        dimensions[0] as f32,
                        dimensions[1] as f32
                    ],
                },
                Scissor::irrelevant()
            )],
        },
        raster: Default::default(),
        multisample: Multisample::disabled(),
        fragment_shader: fs.main_entry_point(),
        depth_stencil: DepthStencil::disabled(),
        blend: Blend::pass_through(),
        render_pass: Subpass::from(target.render_pass().clone(), 0).unwrap(),
    };

    Arc::new(GraphicsPipeline::new(target.device().clone(), pipeline_params).unwrap())
        as Arc<GraphicsPipeline<SingleBufferDefinition<ScreenSizeTriVertex>, _, _>>
}

pub struct ScreenSizeTriVertex {
    pub v_position: [f32; 2],
    pub v_uv: [f32; 2],
}

impl_vertex!(ScreenSizeTriVertex, v_position, v_uv);

fn create_projection_view_matrix(target: &mut Target, camera: &Camera) -> Matrix4<f32> {
    let perspective = PerspectiveFov {
        fovy: Rad::full_turn() * 0.25,
        aspect: target.size().x as f32 / target.size().y as f32,
        near: 0.1,
        far: 500.0,
    };
    // Flip the projection upside down, glm expects opengl values, we need vulkan values
    let projection =
        Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0) * Matrix4::from(perspective);
    let view = camera.create_world_to_view_matrix();

    // Combine the projection and the view, we don't need them separately
    projection * view
}
