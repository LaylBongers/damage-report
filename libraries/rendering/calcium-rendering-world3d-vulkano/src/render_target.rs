use std::sync::{Arc};

use cgmath::{Vector2};
use slog::{Logger};
use vulkano::framebuffer::{Subpass, Framebuffer, RenderPassAbstract, FramebufferAbstract};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::depth_stencil::{DepthStencil, Compare};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::image::swapchain::{SwapchainImage};

use calcium_rendering::{Renderer, Viewport, WindowRenderer};
use calcium_rendering_vulkano::{VulkanoTypes, VulkanoRenderer, VulkanoWindowRenderer};
use calcium_rendering_vulkano_shaders::{gbuffer_vs, gbuffer_fs, lighting_vs, lighting_fs};
use calcium_rendering_world3d::{World3DRenderTargetRaw};

use geometry_buffer::{GeometryBuffer};
use {VulkanoWorld3DTypes, VulkanoWorld3DRenderer};

pub struct VulkanoWorld3DRenderTargetRaw {
    pub geometry_buffer: GeometryBuffer,
    pub geometry_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    pub lighting_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,

    window_render_pass: Arc<RenderPassAbstract + Send + Sync>,
    window_framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    window_framebuffers_images_id: usize,

    viewport: Viewport,
}

impl VulkanoWorld3DRenderTargetRaw {
    pub fn window_framebuffer_for(
        &self, image_num: usize,
    ) -> &Arc<FramebufferAbstract + Send + Sync> {
        // Return the framebuffer for this image_num
        &self.window_framebuffers[image_num]
    }

    pub fn resize_framebuffers(
        &mut self, renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer,
        viewport: &Viewport,
    ) {
        // We only need to update the gbuffer if the viewport got updated
        if self.viewport != *viewport {
            self.geometry_buffer = GeometryBuffer::new(
                renderer, viewport,
            );
            self.viewport = viewport.clone();
        }

        // We only need to update the window framebuffer if the window got updated
        let current_images_id = window_renderer.swapchain.images_id();
        if self.window_framebuffers_images_id != current_images_id {
            // Update the window framebuffers
            self.window_framebuffers = create_window_framebuffers(
                window_renderer.swapchain.images(),
                &self.window_render_pass,
            );
            self.window_framebuffers_images_id = current_images_id;
        }
    }
}

impl World3DRenderTargetRaw<VulkanoTypes, VulkanoWorld3DTypes> for VulkanoWorld3DRenderTargetRaw {
    fn new(
        _should_clear: bool,
        renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer,
        _world3d_renderer: &VulkanoWorld3DRenderer,
    ) -> Self {
        // TODO: Implement should_clear

        // Likely the viewport is fullscreen, it will be updated anyways if that's wrong
        let viewport = Viewport::new(Vector2::new(0.0, 0.0), window_renderer.size().cast());
        let geometry_buffer = GeometryBuffer::new(
            renderer, &viewport,
        );

        // TODO: Prevent shader re-loading
        let geometry_pipeline = load_geometry_pipeline(
            renderer.log(), renderer, geometry_buffer.render_pass.clone()
        );

        let color_buffer_format = window_renderer.swapchain.swapchain.format();
        #[allow(dead_code)]
        let window_render_pass = Arc::new(single_pass_renderpass!(renderer.device().clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_buffer_format,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap()) as Arc<RenderPassAbstract + Send + Sync>;

        // TODO: Prevent shader re-loading
        let lighting_pipeline = load_lighting_pipeline(
            renderer, &window_render_pass
        );

        let window_framebuffers = create_window_framebuffers(
            window_renderer.swapchain.images(), &window_render_pass,
        );
        let window_framebuffers_images_id = window_renderer.swapchain.images_id();

        VulkanoWorld3DRenderTargetRaw {
            geometry_buffer,
            geometry_pipeline,
            lighting_pipeline,

            window_render_pass,
            window_framebuffers,
            window_framebuffers_images_id,

            viewport,
        }
    }
}

fn load_geometry_pipeline(
    log: &Logger, renderer: &VulkanoRenderer,
    gbuffer_render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Loading gbuffer shaders");
    let vs = gbuffer_vs::Shader::load(renderer.device().clone()).unwrap();
    let fs = gbuffer_fs::Shader::load(renderer.device().clone()).unwrap();

    // Set up the pipeline itself
    debug!(log, "Creating gbuffer pipeline");
    Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer()
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)

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
    renderer: &VulkanoRenderer,
    window_render_pass: &Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(renderer.log(), "Loading deferred shaders");
    let vs = lighting_vs::Shader::load(renderer.device().clone()).unwrap();
    let fs = lighting_fs::Shader::load(renderer.device().clone()).unwrap();

    // Set up the pipeline itself
    debug!(renderer.log(), "Creating lighting pipeline");
    Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer()
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)

        // Which shaders to use
        .vertex_shader(vs.main_entry_point(), ())
        .fragment_shader(fs.main_entry_point(), ())

        .depth_stencil_disabled()

        .render_pass(Subpass::from(window_render_pass.clone(), 0).unwrap())
        .build(renderer.device().clone()).unwrap()
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<::lighting_renderer::ScreenSizeTriVertex>, _, _>>
}

fn create_window_framebuffers(
    images: &Vec<Arc<SwapchainImage>>,
    render_pass: &Arc<RenderPassAbstract + Send + Sync>,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    images.iter().map(|image| {
        Arc::new(Framebuffer::start(render_pass.clone())
            .add(image.clone()).unwrap()
            .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>
    }).collect()
}
