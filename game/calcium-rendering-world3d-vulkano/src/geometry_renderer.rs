use std::sync::{Arc};

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4};
use collision::{Frustum, Relation};
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

use calcium_rendering_vulkano::{VulkanoRenderBackend};
use calcium_rendering_vulkano_shaders::{gbuffer_vs, gbuffer_fs};
use calcium_rendering_world3d::{Camera, RenderWorld, Entity};

use geometry_buffer::{GeometryBuffer};
use {BackendMeshes};

pub struct GeometryRenderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl GeometryRenderer {
    pub fn new(
        log: &Logger, backend: &VulkanoRenderBackend, geometry_buffer: &GeometryBuffer
    ) -> Self {
        // Set up the shaders and pipelines
        let pipeline = load_pipeline(log, backend, geometry_buffer.render_pass.clone());

        GeometryRenderer {
            pipeline,
        }
    }

    pub fn build_command_buffer(
        &self, log: &Logger,
        backend: &mut VulkanoRenderBackend,
        meshes: &mut BackendMeshes, geometry_buffer: &GeometryBuffer,
        camera: &Camera, world: &RenderWorld,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            backend.device.clone(), backend.graphics_queue.family()
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
        let projection_view = create_projection_view_matrix(backend, camera);

        // Create a culling frustum from that matrix
        let culling_frustum = Frustum::from_matrix4(projection_view).unwrap();

        // Go over everything in the world
        for entity in world.entities() {
            if let &Some(ref entity) = entity {
                command_buffer_builder = self.render_entity(
                    log, entity, backend, meshes,
                    &projection_view, &culling_frustum,
                    command_buffer_builder
                );
            }
        }

        // Finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }

    fn render_entity(
        &self, log: &Logger,
        entity: &Entity,
        backend: &mut VulkanoRenderBackend, meshes: &mut BackendMeshes,
        projection_view: &Matrix4<f32>,
        culling_frustum: &Frustum<f32>,
        command_buffer: AutoCommandBufferBuilder,
    ) -> AutoCommandBufferBuilder {
        // TODO: The creation of the backend data for textures and meshes should be started
        //  immediately when the Mesh and Texture structures are created. Currently it's being
        //  loaded lazily below. This results in unexpected lag which we don't want.

        // Retrieve the backend textures for the frontend textures, but early-bail if we have any
        //  textures that aren't uploaded yet, this notifies the backend that they should be queued
        //  up for uploading if they aren't yet as well.
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

        // Retrieve the backend mesh for the frontend mesh
        let mesh = if let Some(mesh) = meshes.request_mesh(&log, backend, &entity.mesh) {
            mesh
        } else {
            return command_buffer;
        };

        // Check if this entity's mesh is visible to the current camera
        let mut culling_sphere = mesh.culling_sphere;
        culling_sphere.center.x += entity.position.x;
        culling_sphere.center.y += entity.position.y;
        culling_sphere.center.z += entity.position.z;
        if culling_frustum.contains(culling_sphere) == Relation::Out {
            // It's not visible, so don't make any attempt at rendering it
            return command_buffer;
        }

        // Create a matrix for this world entity
        let model = Matrix4::from_translation(entity.position);
        let total_matrix_raw: [[f32; 4]; 4] = (projection_view * model).into();
        let model_matrix_raw: [[f32; 4]; 4] = model.into();

        // Send the matrices over to the GPU
        // TODO: Instead of creating a new buffer, re-use the descriptor set and overwrite the same
        //  buffer's data (update_buffer)
        let matrix_data_buffer = CpuAccessibleBuffer::<gbuffer_vs::ty::MatrixData>::from_data(
            backend.device.clone(), BufferUsage::all(),
            Some(backend.graphics_queue.family()),
            gbuffer_vs::ty::MatrixData {
                total: total_matrix_raw,
                model: model_matrix_raw,
            }
        ).unwrap();

        // Create the final uniforms set
        // TODO: Re-use the descriptor set for this entity across frames
        let set = Arc::new(simple_descriptor_set!(self.pipeline.clone(), 0, {
            u_matrix_data: matrix_data_buffer,
            u_material_base_color: base_color,
            u_material_normal_map: normal_map,
            u_material_metallic_map: metallic_map,
            u_material_roughness_map: roughness_map,
        }));

        // Perform the actual draw
        // TODO: Investigate the possibility of using draw_indexed_indirect (when it's added to
        //  vulkano)
        command_buffer
            .draw_indexed(
                self.pipeline.clone(), DynamicState::none(),
                vec!(mesh.vertex_buffer.clone()), mesh.index_buffer.clone(),
                set, ()
            ).unwrap()
    }
}

fn load_pipeline(
    log: &Logger, backend: &VulkanoRenderBackend,
    gbuffer_render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading gbuffer shaders");
    let vs = gbuffer_vs::Shader::load(&backend.device).unwrap();
    let fs = gbuffer_fs::Shader::load(&backend.device).unwrap();

    // Set up the pipeline
    debug!(log, "Creating gbuffer pipeline");
    let dimensions = backend.size;
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

    Arc::new(GraphicsPipeline::new(backend.device.clone(), pipeline_params).unwrap())
        as Arc<GraphicsPipeline<SingleBufferDefinition<::VkVertex>, _, _>>
}

fn create_projection_view_matrix(
    backend: &VulkanoRenderBackend, camera: &Camera
) -> Matrix4<f32> {
    let perspective = PerspectiveFov {
        fovy: Rad::full_turn() * 0.1638,
        aspect: backend.size.x as f32 / backend.size.y as f32,
        near: 0.1,
        far: 500.0,
    };
    // Flip the projection upside down, cgmath expects opengl values, we need vulkan values
    let projection = Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0) * Matrix4::from(perspective);
    let view = camera.create_world_to_view_matrix();

    // Combine the projection and the view, we don't need them separately
    projection * view
}
