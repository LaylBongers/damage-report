use std::sync::{Arc};

use slog::{Logger};
use vulkano::format::{ClearValue};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder, DynamicState};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineParams, GraphicsPipelineAbstract};
use vulkano::pipeline::blend::{Blend};
use vulkano::pipeline::depth_stencil::{DepthStencil};
use vulkano::pipeline::input_assembly::{InputAssembly};
use vulkano::pipeline::multisample::{Multisample};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{ViewportsState, Viewport, Scissor};
use vulkano::framebuffer::{Subpass};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};

use calcium_rendering::{Target};
use calcium_rendering_vulkano::{VulkanoBackend, Frame};
use calcium_rendering_vulkano_shaders::{lighting_vs, lighting_fs};
use geometry_buffer::{GeometryBuffer};
use {Camera, World};

pub struct LightingRenderer {
    lighting_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    sampler: Arc<Sampler>,
}

impl LightingRenderer {
    pub fn new(log: &Logger, target: &Target<VulkanoBackend>) -> Self {
        let lighting_pipeline = load_lighting_pipeline(log, target);

        // Create a sampler that we'll use to sample the gbuffer images, this will map 1:1, so just
        //  use nearest. TODO: Because it's 1:1 we can move the gbuffer-lighting steps to subpasses
        let sampler = Sampler::new(
            target.backend().device().clone(),
            Filter::Nearest,
            Filter::Nearest,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).unwrap();

        LightingRenderer {
            lighting_pipeline,
            sampler,
        }
    }

    pub fn build_command_buffer(
        &mut self, target: &mut Target<VulkanoBackend>, frame: &Frame, geometry_buffer: &GeometryBuffer,
        camera: &Camera, world: &World,
    ) -> AutoCommandBufferBuilder {
        let mut command_buffer_builder = AutoCommandBufferBuilder::new(
            target.backend().device().clone(), target.backend().graphics_queue().family()
        ).unwrap();
        // TODO: This method of lighting uses a full-screen tri with all lights passed to it in a
        //  big array. Instead, we should render using "light volumes", which just means rendering
        //  spheres where the light should be one light at a time with the light information, and
        //  blend light data additively from those passes. That should improve performance further.
        //  Instead of using UVs for that, just use screen coordinates. We should also use
        //  instancing to render the spheres, and just use the uniforms to change their size.

        // Begin by starting the render pass, we're rendering the lighting pass directly to the
        //  final framebuffer for this frame, that framebuffer will be presented to the screen.
        // Because this is the final screen framebuffer all we need to clear is the color and
        //  depth. We still use depth because we may want to do another forward render pass for
        //  transparent objects.
        // TODO: Actually make sure the depth ends up in the framebuffer, either through copying or
        //  by directly using this depth attachment during gbuffer rendering.
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
            target.backend().device().clone(), BufferUsage::all(),
            Some(target.backend().graphics_queue().family()),
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
            target.backend().device().clone(), BufferUsage::all(),
            Some(target.backend().graphics_queue().family()),
            lighting_fs::ty::LightData {
                camera_position: camera.position.into(),
                _dummy0: Default::default(),
                ambient_light: world.ambient_light().into(),
                point_lights_amount,
                point_lights,
            }
        ).unwrap();

        // Fill the uniforms set with all the gbuffer images
        let set = Arc::new(simple_descriptor_set!(self.lighting_pipeline.clone(), 0, {
            u_gbuffer_position: (
                geometry_buffer.position_attachment.clone(), self.sampler.clone()
            ),
            u_gbuffer_base_color: (
                geometry_buffer.base_color_attachment.clone(), self.sampler.clone()
            ),
            u_gbuffer_normal: (
                geometry_buffer.normal_attachment.clone(), self.sampler.clone()
            ),
            u_gbuffer_roughness: (
                geometry_buffer.roughness_attachment.clone(), self.sampler.clone()
            ),
            u_gbuffer_metallic: (
                geometry_buffer.metallic_attachment.clone(), self.sampler.clone()
            ),
            u_light_data: light_data_buffer,
        }));

        // Submit the triangle for rendering
        command_buffer_builder = command_buffer_builder
            .draw(
                self.lighting_pipeline.clone(), DynamicState::none(), vec!(sst_buffer), set, ()
            ).unwrap();

        // Finally, finish the render pass
        command_buffer_builder.end_render_pass().unwrap()
    }
}

fn load_lighting_pipeline(
    log: &Logger, target: &Target<VulkanoBackend>
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading deferred shaders");
    let vs = lighting_vs::Shader::load(target.backend().device()).unwrap();
    let fs = lighting_fs::Shader::load(target.backend().device()).unwrap();

    // Set up the pipeline
    debug!(log, "Creating deferred pipeline");
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
                        dimensions.x as f32,
                        dimensions.y as f32
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
        render_pass: Subpass::from(target.backend().swapchain().render_pass.clone(), 0).unwrap(),
    };

    Arc::new(GraphicsPipeline::new(target.backend().device().clone(), pipeline_params).unwrap())
        as Arc<GraphicsPipeline<SingleBufferDefinition<ScreenSizeTriVertex>, _, _>>
}

pub struct ScreenSizeTriVertex {
    pub v_position: [f32; 2],
    pub v_uv: [f32; 2],
}

impl_vertex!(ScreenSizeTriVertex, v_position, v_uv);
