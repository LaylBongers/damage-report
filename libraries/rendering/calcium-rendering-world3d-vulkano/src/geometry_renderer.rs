use std::sync::{Arc};

use cgmath::{Matrix4, Vector2};
use collision::{Frustum, Relation};
use vulkano::format::{ClearValue};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::pipeline::viewport::{Viewport as ViewportVk};

use calcium_rendering::{Error, Viewport, Renderer};
use calcium_rendering::texture::{Texture};
use calcium_rendering_vulkano::{VulkanoRenderer};
use calcium_rendering_vulkano_shaders::{gbuffer_vs};
use calcium_rendering_world3d::{Camera, RenderWorld, Entity, World3DRenderTarget};

use {VulkanoWorld3DRenderer};

pub struct GeometryRenderer {
    default_black: Arc<Texture<VulkanoRenderer>>,
    default_white: Arc<Texture<VulkanoRenderer>>,
    default_normal: Arc<Texture<VulkanoRenderer>>,
}

impl GeometryRenderer {
    pub fn new(
        renderer: &mut VulkanoRenderer,
    ) -> Result<Self, Error> {
        const BLACK_TEXTURE_1PX: &[u8] = &[0];
        const WHITE_TEXTURE_1PX: &[u8] = &[255u8];
        const NORMAL_TEXTURE_1PX: &[u8] = &[126u8, 126u8, 255u8, 255u8];

        info!(renderer.log(), "Creating default material textures");
        let default_black = Texture::new()
            .from_bytes(BLACK_TEXTURE_1PX, Vector2::new(1, 1), false)
            .as_single_channel()
            .build(renderer)?;
        let default_white = Texture::new()
            .from_bytes(WHITE_TEXTURE_1PX, Vector2::new(1, 1), false)
            .as_single_channel()
            .build(renderer)?;
        let default_normal = Texture::new()
            .from_bytes(NORMAL_TEXTURE_1PX, Vector2::new(1, 1), true)
            .as_linear()
            .build(renderer)?;

        Ok(GeometryRenderer {
            default_black,
            default_white,
            default_normal,
        })
    }

    pub fn build_command_buffer(
        &self,
        world: &RenderWorld<VulkanoRenderer, VulkanoWorld3DRenderer>, camera: &Camera,
        rendertarget: &mut World3DRenderTarget<VulkanoRenderer, VulkanoWorld3DRenderer>,
        renderer: &mut VulkanoRenderer,
        viewport: &Viewport,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            renderer.device().clone(), renderer.graphics_queue().family()
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
            ClearValue::Float([0.0, 0.0, 0.0, 1.0]),
            ClearValue::Depth(0.0)
        );
        command_buffer_builder = command_buffer_builder.begin_render_pass(
            rendertarget.raw.geometry_buffer.framebuffer.clone(), false, clear_values
        ).unwrap();

        // Create the projection-view matrix needed for the perspective rendering
        let projection_view = camera.world_to_screen_matrix(viewport);

        // Create a culling frustum from that matrix
        let culling_frustum = Frustum::from_matrix4(projection_view).unwrap();

        // Go over everything in the world
        for entity in world.entities() {
            if let &Some(ref entity) = entity {
                command_buffer_builder = self.render_entity(
                    entity,
                    rendertarget,
                    renderer,
                    &projection_view, &culling_frustum,
                    command_buffer_builder,
                    viewport,
                );
            }
        }

        // Finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }

    fn render_entity(
        &self,
        entity: &Entity<VulkanoRenderer, VulkanoWorld3DRenderer>,
        rendertarget: &mut World3DRenderTarget<VulkanoRenderer, VulkanoWorld3DRenderer>,
        renderer: &mut VulkanoRenderer,
        projection_view: &Matrix4<f32>, culling_frustum: &Frustum<f32>,
        command_buffer: AutoCommandBufferBuilder,
        viewport: &Viewport,
    ) -> AutoCommandBufferBuilder {
        // Check if this entity's mesh is visible to the current camera
        let mut culling_sphere = entity.mesh.raw.culling_sphere;
        culling_sphere.center.x += entity.position.x;
        culling_sphere.center.y += entity.position.y;
        culling_sphere.center.z += entity.position.z;
        if culling_frustum.contains(&culling_sphere) == Relation::Out {
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
            renderer.device().clone(), BufferUsage::all(),
            gbuffer_vs::ty::MatrixData {
                total: total_matrix_raw,
                model: model_matrix_raw,
            }
        ).unwrap();

        // Create the final uniforms set
        let material = &entity.material;
        let base_color = material.base_color.as_ref().unwrap_or(&self.default_white);
        let normal_map = material.normal_map.as_ref().unwrap_or(&self.default_normal);
        let metallic_map = material.metallic_map.as_ref().unwrap_or(&self.default_black);
        let roughness_map = material.roughness_map.as_ref().unwrap_or(&self.default_white);
        let ambient_occlusion_map = material.ambient_occlusion_map
            .as_ref().unwrap_or(&self.default_white);

        let set = Arc::new(rendertarget.raw.geometry_set_pool.next()
            .add_buffer(matrix_data_buffer.clone()).unwrap()
            .add_sampled_image(
                base_color.raw.image().clone(), base_color.raw.sampler().clone()
            ).unwrap()
            .add_sampled_image(
                normal_map.raw.image().clone(), normal_map.raw.sampler().clone()
            ).unwrap()
            .add_sampled_image(
                metallic_map.raw.image().clone(), metallic_map.raw.sampler().clone()
            ).unwrap()
            .add_sampled_image(
                roughness_map.raw.image().clone(), roughness_map.raw.sampler().clone()
            ).unwrap()
            .add_sampled_image(
                ambient_occlusion_map.raw.image().clone(),
                ambient_occlusion_map.raw.sampler().clone()
            ).unwrap()
            .build().unwrap()
        );

        // Perform the actual draw
        // TODO: Investigate the possibility of using draw_indexed_indirect (when it's added to
        //  vulkano)
        command_buffer
            .draw_indexed(
                rendertarget.raw.geometry_pipeline.clone(),
                // TODO: When a lot is being rendered, check the performance impact of doing
                //  this here instead of in the pipeline.
                DynamicState {
                    viewports: Some(vec!(ViewportVk {
                        origin: [0.0, 0.0],
                        depth_range: 0.0 .. 1.0,
                        dimensions: viewport.size.into(),
                    })),
                    .. DynamicState::none()
                },
                vec!(entity.mesh.raw.vertex_buffer.clone()),
                entity.mesh.raw.index_buffer.clone(),
                set, ()
            ).unwrap()
    }
}
