use std::sync::{Arc};
use std::iter;

use cgmath::{Rad, Angle, Matrix4};
use collision::{Frustum, Relation};
use slog::{Logger};
use vulkano::format::{ClearValue};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder, DynamicState};
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::depth_stencil::{DepthStencil, Compare};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use calcium_rendering_vulkano::{VulkanoBackendTypes, VulkanoRenderBackend};
use calcium_rendering_vulkano_shaders::{gbuffer_vs, gbuffer_fs};
use calcium_rendering_world3d::{Camera, RenderWorld, Entity};

use geometry_buffer::{GeometryBuffer};
use {VulkanoWorldBackendTypes};

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
        &self,
        backend: &mut VulkanoRenderBackend,
        geometry_buffer: &GeometryBuffer,
        camera: &Camera, world: &RenderWorld<VulkanoBackendTypes, VulkanoWorldBackendTypes>,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            backend.device.clone(), backend.graphics_queue.family()
        ).unwrap();

        let clear_values = vec!(
            // These colors has no special significance, it's just useful for debugging that a lack
            //  of a value is set to black.
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            // 0.0 alpha so we can discard unused pixels
            // TODO: Replace that discard test with emissive color, see shader for info why
            ClearValue::Float([0.0, 0.0, 0.0, 0.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Depth(0.0)
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
                    entity,
                    backend,
                    &projection_view, &culling_frustum,
                    command_buffer_builder
                );
            }
        }

        // Finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }

    fn render_entity(
        &self,
        entity: &Entity<VulkanoBackendTypes, VulkanoWorldBackendTypes>,
        backend: &mut VulkanoRenderBackend,
        projection_view: &Matrix4<f32>, culling_frustum: &Frustum<f32>,
        command_buffer: AutoCommandBufferBuilder,
    ) -> AutoCommandBufferBuilder {
        // Check if this entity's mesh is visible to the current camera
        let mut culling_sphere = entity.mesh.backend.culling_sphere;
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
            u_material_base_color: entity.material.base_color.backend.uniform(),
            u_material_normal_map: entity.material.normal_map.backend.uniform(),
            u_material_metallic_map: entity.material.metallic_map.backend.uniform(),
            u_material_roughness_map: entity.material.roughness_map.backend.uniform(),
        }));

        // Perform the actual draw
        // TODO: Investigate the possibility of using draw_indexed_indirect (when it's added to
        //  vulkano)
        command_buffer
            .draw_indexed(
                self.pipeline.clone(), DynamicState::none(),
                vec!(entity.mesh.backend.vertex_buffer.clone()),
                entity.mesh.backend.index_buffer.clone(),
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

    // Set up the pipeline itself
    debug!(log, "Creating gbuffer pipeline");
    let dimensions = backend.size;
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

        // Cull back faces
        .cull_mode_back()
        .front_face_counter_clockwise()

        // Reverse-Z depth testing
        .depth_stencil(DepthStencil {
            depth_compare: Compare::Greater,
            .. DepthStencil::simple_depth_test()
        })

        .render_pass(Subpass::from(gbuffer_render_pass, 0).unwrap())
        .build(backend.device.clone()).unwrap()
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<::mesh::VkVertex>, _, _>>
}

fn create_projection_view_matrix(
    backend: &VulkanoRenderBackend, camera: &Camera
) -> Matrix4<f32> {
    // Create the projection matrix, which is what makes this a 3D perspective renderer
    let y_fov = Rad::full_turn() * 0.1638; // 90 deg x-fov for this aspect ratio
    let aspect = backend.size.x as f32 / backend.size.y as f32;
    let projection = create_infinity_projection(y_fov, aspect, 0.1);

    // Combine the projection and the view, we don't need them separately
    let view = camera.create_world_to_view_matrix();
    projection * view
}

/// This projection function creates a "Reverse-Z Infinity Far Plane" projection. It has various
/// advantages over a traditional forward Z near/far projection.
///
/// The reverse Z improves precision on floating point depth buffers, because the Z in projection
/// matrices isn't linear, values near the camera will take up a lot more of the number line than
/// values far away will. Reverse-Z allows values far away to use floating point values closer to
/// zero, taking advantage of the ability of floating point values to adjust precision. This will
/// give identical results for integer depth buffers, so we might as well make use of it.
///
/// The infinity far plane makes it much easier to create games with extremely long view distances.
/// It also means you don't actually have to worry about the far clipping plane removing things
/// you want on screen.
///
/// This projection matrix gives depth values in the 0..1 range, and Y values matching Vulkan's
/// screen space (Y is down).
fn create_infinity_projection(y_fov: Rad<f32>, aspect: f32, z_near: f32) -> Matrix4<f32> {
    let f = 1.0 / (y_fov.0 / 2.0).tan();
    Matrix4::new(
        f / aspect, 0.0,  0.0,  0.0,
        0.0, -f, 0.0, 0.0,
        0.0, 0.0, 0.0, -1.0,
        0.0, 0.0, z_near, 0.0
    )
}
