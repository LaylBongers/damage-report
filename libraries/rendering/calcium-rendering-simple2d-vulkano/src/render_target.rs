use std::sync::{Arc};

use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::framebuffer::{Subpass, RenderPassAbstract, Framebuffer, FramebufferAbstract};
use vulkano::format::{ClearValue};
use vulkano::image::swapchain::{SwapchainImage};
use vulkano::descriptor::descriptor_set::{FixedSizeDescriptorSetsPool};

use calcium_rendering::{Renderer};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoWindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderTargetRaw};

use {VkVertex, VulkanoSimple2DRenderer};

pub struct VulkanoSimple2DRenderTargetRaw {
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    set_pool: FixedSizeDescriptorSetsPool<Arc<GraphicsPipelineAbstract + Send + Sync>>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    should_clear: bool,

    framebuffers_images_id: usize,
}

impl VulkanoSimple2DRenderTargetRaw {
    pub fn pipeline(&self) -> &Arc<GraphicsPipelineAbstract + Send + Sync> {
        &self.pipeline
    }

    pub fn set_pool_mut(
        &mut self
    ) -> &mut FixedSizeDescriptorSetsPool<Arc<GraphicsPipelineAbstract + Send + Sync>> {
        &mut self.set_pool
    }

    pub fn framebuffer_for(
        &mut self, image_num: usize, window_renderer: &VulkanoWindowRenderer,
    ) -> &Arc<FramebufferAbstract + Send + Sync> {
        // Check if we should update the framebuffers
        let current_images_id = window_renderer.swapchain.images_id();
        if self.framebuffers_images_id != current_images_id {
            self.framebuffers = create_framebuffers(
                window_renderer.swapchain.images(),
                &self.render_pass,
            );
            self.framebuffers_images_id = current_images_id;
        }

        // Return the framebuffer for this image_num
        &self.framebuffers[image_num]
    }

    pub fn clear_values(&self) -> Vec<ClearValue> {
        if self.should_clear {
            vec!(ClearValue::Float([0.0, 0.0, 0.0, 1.0]))
        } else {
            vec!(ClearValue::None)
        }
    }
}

impl Simple2DRenderTargetRaw<VulkanoRenderer, VulkanoSimple2DRenderer>
    for VulkanoSimple2DRenderTargetRaw {
    fn new(
        should_clear: bool,
        renderer: &VulkanoRenderer, window_renderer: &VulkanoWindowRenderer,
        simple2d_renderer: &VulkanoSimple2DRenderer,
    ) -> Self {
        // Set up the render pass for 2D rendering depending on the settings for this target
        debug!(renderer.log(), "Creating simple2d render pass");
        #[allow(dead_code)]
        let render_pass = if should_clear {
            Arc::new(single_pass_renderpass!(renderer.device().clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        // TODO: Get this format from a central place that isn't the window
                        format: ::vulkano::format::Format::B8G8R8A8Srgb,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            ).unwrap()) as Arc<RenderPassAbstract + Send + Sync>
        } else {
            Arc::new(single_pass_renderpass!(renderer.device().clone(),
                attachments: {
                    color: {
                        load: Load,
                        store: Store,
                        // TODO: Get this format from a central place that isn't the window
                        format: ::vulkano::format::Format::B8G8R8A8Srgb,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            ).unwrap()) as Arc<RenderPassAbstract + Send + Sync>
        };

        // Set up the pipeline for this target
        debug!(renderer.log(), "Creating simple2d pipeline");
        let pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)

            // Which shaders to use
            .vertex_shader(simple2d_renderer.vs.main_entry_point(), ())
            .fragment_shader(simple2d_renderer.fs.main_entry_point(), ())

            .blend_alpha_blending()
            .cull_mode_disabled()

            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(renderer.device().clone()).unwrap()
        ) as Arc<GraphicsPipeline<SingleBufferDefinition<VkVertex>, _, _>>;

        // Create specialized set pools for more efficient rendering
        let set_pool = FixedSizeDescriptorSetsPool::new(
            pipeline.clone() as Arc<GraphicsPipelineAbstract + Send + Sync>,
            0
        );

        // Create the swapchain framebuffers for this render pass
        let framebuffers = create_framebuffers(
            window_renderer.swapchain.images(),
            &render_pass,
        );
        let framebuffers_images_id = window_renderer.swapchain.images_id();

        VulkanoSimple2DRenderTargetRaw {
            render_pass,
            pipeline,
            set_pool,
            framebuffers,
            should_clear,
            framebuffers_images_id,
        }
    }
}

fn create_framebuffers(
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
