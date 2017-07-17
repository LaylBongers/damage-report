use std::sync::{Arc};
use std::iter;

use slog::{Logger};
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::depth_stencil::{DepthStencil, Compare};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{Viewport};

use calcium_rendering::{Renderer, WindowRenderer};
use calcium_rendering_vulkano::{VulkanoTypes, VulkanoRenderer, VulkanoWindowRenderer};
use calcium_rendering_vulkano_shaders::{gbuffer_vs, gbuffer_fs, lighting_vs, lighting_fs};
use calcium_rendering_world3d::{World3DRenderTargetRaw};

use geometry_buffer::{GeometryBuffer};
use {VulkanoWorld3DTypes, VulkanoWorld3DRenderer};

pub struct VulkanoWorld3DRenderTargetRaw {
    pub geometry_buffer: GeometryBuffer,
    pub geometry_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    pub lighting_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl World3DRenderTargetRaw<VulkanoTypes, VulkanoWorld3DTypes> for VulkanoWorld3DRenderTargetRaw {
    fn new(
        _should_clear: bool,
        renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer,
        _world3d_renderer: &VulkanoWorld3DRenderer,
    ) -> Self {
        let geometry_buffer = GeometryBuffer::new(
            renderer, window_renderer, window_renderer.swapchain.depth_attachment.clone()
        );

        // TODO: Prevent shader re-loading
        let geometry_pipeline = load_geometry_pipeline(
            renderer.log(), renderer, window_renderer, geometry_buffer.render_pass.clone()
        );

        let lighting_pipeline = load_lighting_pipeline(renderer, window_renderer);

        VulkanoWorld3DRenderTargetRaw {
            geometry_buffer,
            geometry_pipeline,
            lighting_pipeline,
        }
    }
}

fn load_geometry_pipeline(
    log: &Logger, renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer,
    gbuffer_render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading gbuffer shaders");
    let vs = gbuffer_vs::Shader::load(renderer.device().clone()).unwrap();
    let fs = gbuffer_fs::Shader::load(renderer.device().clone()).unwrap();

    // Set up the pipeline itself
    debug!(log, "Creating gbuffer pipeline");
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

        // Cull back faces
        .cull_mode_back()
        .front_face_counter_clockwise()

        // Reverse-Z depth testing
        .depth_stencil(DepthStencil {
            depth_compare: Compare::Greater,
            .. DepthStencil::simple_depth_test()
        })

        .render_pass(Subpass::from(gbuffer_render_pass, 0).unwrap())
        .build(renderer.device().clone()).unwrap()
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<::mesh::VkVertex>, _, _>>
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
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<::lighting_renderer::ScreenSizeTriVertex>, _, _>>
}
