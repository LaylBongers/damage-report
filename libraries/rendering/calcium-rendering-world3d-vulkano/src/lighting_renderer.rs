use std::sync::{Arc};
use std::iter;

use vulkano::format::{ClearValue};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::framebuffer::{Subpass};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet};

use calcium_rendering::{Renderer, WindowRenderer};
use calcium_rendering_vulkano::{VulkanoTypes, VulkanoRenderer, VulkanoFrame, VulkanoWindowRenderer};
use calcium_rendering_vulkano_shaders::{lighting_vs, lighting_fs};
use calcium_rendering_world3d::{Camera, RenderWorld};

use geometry_buffer::{GeometryBuffer};
use {VulkanoWorld3DTypes};

pub struct LightingRenderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    sampler: Arc<Sampler>,
}

impl LightingRenderer {
    pub fn new(
        renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer
    ) -> Self {
        let pipeline = load_lighting_pipeline(renderer, window_renderer);

        // Create a sampler that we'll use to sample the gbuffer images, this will map 1:1, so just
        //  use nearest. TODO: Because it's 1:1 we can move the gbuffer-lighting steps to subpasses
        let sampler = Sampler::new(
            renderer.device().clone(),
            Filter::Nearest,
            Filter::Nearest,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).unwrap();

        LightingRenderer {
            pipeline,
            sampler,
        }
    }

    pub fn build_command_buffer(
        &mut self,
        renderer: &mut VulkanoRenderer, frame: &VulkanoFrame, geometry_buffer: &GeometryBuffer,
        camera: &Camera, world: &RenderWorld<VulkanoTypes, VulkanoWorld3DTypes>,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            renderer.device().clone(), renderer.graphics_queue().family()
        ).unwrap();
        // TODO: This method of lighting uses a full-screen tri with all lights passed to it in a
        //  big array. Instead, we should render using "light volumes", which just means rendering
        //  spheres where the light should be one light at a time with the light information, and
        //  blend light data additively from those passes. That should improve performance further.
        //  Instead of using UVs for that, just use screen coordinates. We should also use
        //  instancing to render the spheres, and just use the uniforms to change their size.

        // Begin by starting the render pass, we're rendering the lighting pass directly to the
        //  final framebuffer for this frame, that framebuffer will be presented to the screen.
        // TODO: Actually make sure the depth ends up in the framebuffer, we're already using the
        //  depth buffer during geometry rendering but now we're clearing it, we still need it for
        //  further transparent render passes.
        let clear_values = vec!(
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
            renderer.device().clone(), BufferUsage::all(),
            Some(renderer.graphics_queue().family()),
            sst_vertices.into_iter()
        ).unwrap();

        // Initialize the light array, we need to say how many lights we have and fill it with
        //  dummy values, we'll add actual light data in the next steps
        let point_lights_amount = world.lights().len() as i32;
        let mut point_lights = [lighting_fs::ty::PointLight {
            position: [0.0, 0.0, 0.0],
            _dummy0: Default::default(),
            color: [0.0, 0.0, 0.0],
            inverse_radius_sqr: 0.0,
        }; 32];

        // Make sure we're not going over the maximum amount of lights
        if point_lights_amount > 32 {
            panic!("Currently a maximum of 32 lights is supported");
        }

        // Fill the actual light data
        for i in 0..point_lights_amount as usize {
            let light = &world.lights()[i];
            point_lights[i].position = light.position.into();
            point_lights[i].color = light.color.into();
            let inverse_radius = 1.0 / light.radius;
            point_lights[i].inverse_radius_sqr = inverse_radius * inverse_radius;
        }

        // Create a buffer with all the lighting data, so we can send it over to the shader which
        //  needs this data to actually calculate the light for every pixel.
        let light_data_buffer = CpuAccessibleBuffer::<lighting_fs::ty::LightData>::from_data(
            renderer.device().clone(), BufferUsage::all(),
            Some(renderer.graphics_queue().family()),
            lighting_fs::ty::LightData {
                _dummy0: Default::default(),
                _dummy1: Default::default(),
                _dummy2: Default::default(),
                camera_position: camera.position.into(),
                ambient_color: world.ambient_light.into(),
                directional_color: world.directional_light.into(),
                directional_direction: world.directional_direction.into(),
                point_lights_amount,
                point_lights,
            }
        ).unwrap();

        // Fill the uniforms set with all the gbuffer images
        // TODO: We can probably avoid creating this set every time
        let set = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
            .add_sampled_image(
                geometry_buffer.position_attachment.clone(), self.sampler.clone()
            ).unwrap()
            .add_sampled_image(
                geometry_buffer.base_color_attachment.clone(), self.sampler.clone()
            ).unwrap()
            .add_sampled_image(
                geometry_buffer.normal_attachment.clone(), self.sampler.clone()
            ).unwrap()
            .add_sampled_image(
                geometry_buffer.roughness_attachment.clone(), self.sampler.clone()
            ).unwrap()
            .add_sampled_image(
                geometry_buffer.metallic_attachment.clone(), self.sampler.clone()
            ).unwrap()
            .add_buffer(light_data_buffer.clone()).unwrap()
            .build().unwrap()
        );

        // Submit the triangle for rendering
        command_buffer_builder = command_buffer_builder
            .draw(
                self.pipeline.clone(), DynamicState::none(), vec!(sst_buffer), set, ()
            ).unwrap();

        // Finally, finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }
}

fn load_lighting_pipeline(
    renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(renderer.log(), "Loading deferred shaders");
    let vs = lighting_vs::Shader::load(renderer.device().clone()).unwrap();
    let fs = lighting_fs::Shader::load(renderer.device().clone()).unwrap();

    // Set up the pipeline itself
    debug!(renderer.log(), "Creating lighting pipeline");
    let dimensions = window_renderer.size();
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

        .depth_stencil_disabled()

        .render_pass(Subpass::from(window_renderer.swapchain.render_pass.clone(), 0).unwrap())
        .build(renderer.device().clone()).unwrap()
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<ScreenSizeTriVertex>, _, _>>
}

pub struct ScreenSizeTriVertex {
    pub v_position: [f32; 2],
    pub v_uv: [f32; 2],
}

impl_vertex!(ScreenSizeTriVertex, v_position, v_uv);
