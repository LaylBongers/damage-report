use std::sync::{Arc};

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4};
use slog::{Logger};
use vulkano::format::{ClearValue};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder, DynamicState};
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineParams, GraphicsPipelineAbstract};
use vulkano::pipeline::depth_stencil::{DepthStencil};
use vulkano::pipeline::input_assembly::{InputAssembly};
use vulkano::pipeline::multisample::{Multisample};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{ViewportsState, Viewport, Scissor};
use vulkano::pipeline::raster::{Rasterization, CullMode, FrontFace};
use vulkano::pipeline::blend::{Blend};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use calcium_rendering::{Target};
use calcium_rendering_vulkano::{VulkanoBackend};
use calcium_rendering_vulkano_shaders::{gbuffer_vs, gbuffer_fs};
use geometry_buffer::{GeometryBuffer};
use {Camera, World, Entity};

pub struct GeometryRenderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl GeometryRenderer {
    pub fn new(log: &Logger, target: &Target<VulkanoBackend>, geometry_buffer: &GeometryBuffer) -> Self {
        // Set up the shaders and pipelines
        let pipeline = load_pipeline(log, target, geometry_buffer.render_pass.clone());

        GeometryRenderer {
            pipeline,
        }
    }

    pub fn build_command_buffer(
        &mut self, log: &Logger,
        target: &mut Target<VulkanoBackend>, geometry_buffer: &GeometryBuffer,
        camera: &Camera, world: &World,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            target.backend().device().clone(), target.backend().graphics_queue().family()
        ).unwrap();

        let clear_values = vec!(
            // These colors has no special significance, it's just useful for debugging that a lack
            //  of a value is set to black.
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            // 0.0 alpha so we can discard unused pixels
            // TODO: Replace with emissive color, see shader for info why
            ClearValue::Float([0.0, 0.0, 0.0, 0.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Depth(1.0)
        );
        command_buffer_builder = command_buffer_builder
            .begin_render_pass(geometry_buffer.framebuffer.clone(), false, clear_values).unwrap();

        // Create the projection-view matrix needed for the perspective rendering
        let projection_view = create_projection_view_matrix(target, camera);

        // Go over everything in the world
        for entity in world.entities() {
            command_buffer_builder = self.render_entity(
                log, entity, target, &projection_view, command_buffer_builder
            );
        }

        // Finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }

    fn render_entity(
        &self, log: &Logger,
        entity: &Entity, target: &mut Target<VulkanoBackend>,
        projection_view: &Matrix4<f32>,
        command_buffer: AutoCommandBufferBuilder,
    ) -> AutoCommandBufferBuilder {
        let backend = target.backend_mut();

        // Retrieve the backend textures for the frontend textures, but early-bail if we have any
        //  textures that aren't uploaded yet, this notifies the backend that they should be queued
        //  up for uploading if they aren't yet as well.
        // TODO: Check all textures before returning, so they're all submitted at once
        let (base_color, normal_map, metallic_map, roughness_map) = {
            let base_color = if let Some(base_color) =
                backend.request_texture(log, &entity.material.base_color) {
                base_color
            } else { return command_buffer }.uniform();
            let normal_map = if let Some(normal_map) =
                backend.request_texture(log, &entity.material.normal_map) {
                normal_map
            } else { return command_buffer }.uniform();
            let metallic_map = if let Some(metallic_map) =
                backend.request_texture(log, &entity.material.metallic_map) {
                metallic_map
            } else { return command_buffer }.uniform();
            let roughness_map = if let Some(roughness_map) =
                backend.request_texture(log, &entity.material.roughness_map) {
                roughness_map
            } else { return command_buffer }.uniform();
            (base_color, normal_map, metallic_map, roughness_map)
        };

        // Create a matrix for this world entity
        let model = Matrix4::from_translation(entity.position);
        let total_matrix_raw: [[f32; 4]; 4] = (projection_view * model).into();
        let model_matrix_raw: [[f32; 4]; 4] = model.into();

        // Send the matrices over to the GPU
        let matrix_data_buffer = CpuAccessibleBuffer::<gbuffer_vs::ty::MatrixData>::from_data(
            backend.device().clone(), BufferUsage::all(),
            Some(backend.graphics_queue().family()),
            gbuffer_vs::ty::MatrixData {
                total: total_matrix_raw,
                model: model_matrix_raw,
            }
        ).unwrap();

        // Create the final uniforms set
        let set = Arc::new(simple_descriptor_set!(self.pipeline.clone(), 0, {
            u_matrix_data: matrix_data_buffer,
            u_material_base_color: base_color,
            u_material_normal_map: normal_map,
            u_material_metallic_map: metallic_map,
            u_material_roughness_map: roughness_map,
        }));

        // Perform the actual draw
        command_buffer
            .draw_indexed(
                self.pipeline.clone(), DynamicState::none(),
                vec!(entity.mesh.vertex_buffer.clone()), entity.mesh.index_buffer.clone(),
                set, ()
            ).unwrap()
    }
}

fn load_pipeline(
    log: &Logger, target: &Target<VulkanoBackend>,
    gbuffer_render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading gbuffer shaders");
    let vs = gbuffer_vs::Shader::load(target.backend().device()).unwrap();
    let fs = gbuffer_fs::Shader::load(target.backend().device()).unwrap();

    // Set up the pipeline
    debug!(log, "Creating gbuffer pipeline");
    let dimensions = target.backend().size();
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
        raster: Rasterization {
            cull_mode: CullMode::Back,
            front_face: FrontFace::CounterClockwise,
            .. Default::default()
        },
        multisample: Multisample::disabled(),
        fragment_shader: fs.main_entry_point(),
        depth_stencil: DepthStencil::simple_depth_test(),
        blend: Blend::pass_through(),
        render_pass: Subpass::from(gbuffer_render_pass, 0).unwrap(),
    };

    Arc::new(GraphicsPipeline::new(target.backend().device().clone(), pipeline_params).unwrap())
        as Arc<GraphicsPipeline<SingleBufferDefinition<::VkVertex>, _, _>>
}

fn create_projection_view_matrix(target: &mut Target<VulkanoBackend>, camera: &Camera) -> Matrix4<f32> {
    let perspective = PerspectiveFov {
        fovy: Rad::full_turn() * 0.25,
        aspect: target.backend().size().x as f32 / target.backend().size().y as f32,
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
