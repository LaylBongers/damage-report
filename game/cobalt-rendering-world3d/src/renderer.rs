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

use cobalt_rendering::{Target, Frame};
use {Camera, World, Entity};

pub struct Renderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl Renderer {
    pub fn init(log: &Logger, target: &Target) -> Self {
        info!(log, "Initializing world renderer");

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
        let pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<::Vertex>, _, _>> =
            Arc::new(GraphicsPipeline::new(target.device().clone(), pipeline_params).unwrap());

        Renderer {
            pipeline,
        }
    }

    pub fn render(
        &mut self, target: &mut Target, frame: &mut Frame, camera: &Camera, world: &World
    ) {
        // Start the render pass
        let clear_values = vec!(
            ClearValue::Float([0.005, 0.005, 0.005, 1.0]),
            ClearValue::Depth(1.0)
        );
        frame.command_buffer_builder = Some(frame.command_buffer_builder.take().unwrap()
            .begin_render_pass(
                frame.framebuffer.clone(), false,
                clear_values
            ).unwrap()
        );

        // Create the projection-view matrix needed for the perspective rendering
        let projection_view = Self::create_projection_view(target, camera);

        // Retrieve the one point light
        // TODO: Support variable light amounts
        // TODO: Make use of this value
        assert!(world.lights().len() == 1);
        let light = &world.lights()[0];

        // Send over the lights information to vulkan
        let light_data_buffer = CpuAccessibleBuffer::<fs::ty::LightData>::from_data(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            fs::ty::LightData {
                camera_position: camera.position.into(),
                _dummy0: Default::default(),
                ambient_light: world.ambient_light().into(),
                _dummy1: Default::default(),
                light_position: light.position.into(),
                _dummy2: Default::default(),
                light_color: light.color.into(),
            }
        ).unwrap();

        // Go over everything in the world
        for entity in world.entities() {
            self.render_entity(entity, target, frame, &projection_view, &light_data_buffer);
        }

        // Finish the render pass
        frame.command_buffer_builder = Some(frame.command_buffer_builder.take().unwrap()
            .end_render_pass().unwrap()
        );
    }

    fn create_projection_view(target: &mut Target, camera: &Camera) -> Matrix4<f32> {
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

    fn render_entity(
        &self,
        entity: &Entity, target: &mut Target, frame: &mut Frame,
        projection_view: &Matrix4<f32>,
        light_data_buffer: &Arc<CpuAccessibleBuffer<fs::ty::LightData>>
    ) {
        // Create a matrix for this world entity
        let model = Matrix4::from_translation(entity.position);
        let total_matrix_raw: [[f32; 4]; 4] = (projection_view * model).into();
        let model_matrix_raw: [[f32; 4]; 4] = model.into();

        // Send the matrices over to the GPU
        let matrix_data_buffer = CpuAccessibleBuffer::<vs::ty::MatrixData>::from_data(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            vs::ty::MatrixData {
                total: total_matrix_raw,
                model: model_matrix_raw,
            }
        ).unwrap();

        // Create the final uniforms set
        let set = Arc::new(simple_descriptor_set!(self.pipeline.clone(), 0, {
            u_matrix_data: matrix_data_buffer,
            u_material_base_color: entity.material.base_color.uniform(),
            u_material_normal_map: entity.material.normal_map.uniform(),
            u_light_data: light_data_buffer.clone(),
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
