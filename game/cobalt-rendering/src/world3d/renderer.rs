use std::sync::{Arc};

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4};
use slog::{Logger};
use vulkano::format::{ClearValue};
use vulkano::command_buffer::{CommandBufferBuilder, DynamicState};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineParams, GraphicsPipelineAbstract};
use vulkano::pipeline::blend::{Blend};
use vulkano::pipeline::depth_stencil::{DepthStencil};
use vulkano::pipeline::input_assembly::{InputAssembly};
use vulkano::pipeline::multisample::{Multisample};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{ViewportsState, Viewport, Scissor};
use vulkano::framebuffer::{Subpass};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use cobalt_rendering_shaders::{vs, fs};

use world3d::{Camera, World, Entity};
use {Target, Frame};

pub struct Renderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl Renderer {
    pub fn init(log: &Logger, target: &Target) -> Self {
        info!(log, "Initializing Vulkan 3D world renderer");

        // Load in the shaders
        debug!(log, "Loading shaders");
        let vs = vs::Shader::load(target.device()).unwrap();
        let fs = fs::Shader::load(target.device()).unwrap();

        // Set up a pipeline TODO: Comment better
        debug!(log, "Creating render pipeline");
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
                            target.images()[0].dimensions()[0] as f32,
                            target.images()[0].dimensions()[1] as f32
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
            render_pass: Subpass::from(target.render_pass().clone(), 0).unwrap(),
        };
        let pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<::world3d::Vertex>, _, _>> =
            Arc::new(GraphicsPipeline::new(target.device(), pipeline_params).unwrap());

        Renderer {
            pipeline,
        }
    }

    pub fn render(
        &mut self, target: &mut Target, frame: &mut Frame, camera: &Camera, world: &World
    ) {
        // Then, start the render pass
        let clear_values = vec!(
            ClearValue::Float([0.005, 0.005, 0.006, 1.0]),
            ClearValue::Depth(1.0)
        );
        frame.command_buffer_builder = Some(frame.command_buffer_builder.take().unwrap()
            .begin_render_pass(
                frame.framebuffer.clone(), false,
                clear_values
            ).unwrap()
        );

        // Create the uniforms
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
        let projection_view = projection * view;

        // Go over everything in the world
        for entity in world.entities() {
            self.render_entity(entity, target, frame, &projection_view);
        }

        // End the render pass
        frame.command_buffer_builder = Some(frame.command_buffer_builder.take().unwrap()
            .end_render_pass().unwrap()
        );
    }

    fn render_entity(
        &self,
        entity: &Entity, target: &mut Target, frame: &mut Frame,
        projection_view: &Matrix4<f32>
    ) {
        // Create a matrix for this world entity
        let model = Matrix4::from_translation(entity.position);
        let matrix_raw: [[f32; 4]; 4] = (projection_view * model).into();

        // Send the uniforms over to the GPU
        let uniform_buffer = CpuAccessibleBuffer::<vs::ty::UniformsData>::from_data(
            target.device(), &BufferUsage::all(), Some(target.graphics_queue().family()),
            vs::ty::UniformsData {
                matrix: matrix_raw,
            }
        ).unwrap();
        let set = Arc::new(simple_descriptor_set!(self.pipeline.clone(), 0, {
            u_data: uniform_buffer,
            u_texture: entity.material.uniform(),
        }));

        // Perform the actual draw
        frame.command_buffer_builder = Some(frame.command_buffer_builder.take().unwrap()
            .draw_indexed(
                self.pipeline.clone(), DynamicState::none(),
                vec!(entity.mesh.vertex_buffer.clone()), entity.mesh.index_buffer.clone(),
                set, ()
            ).unwrap()
        );
    }
}
