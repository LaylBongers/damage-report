use std::sync::{Arc};

use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::format::{ClearValue};

use calcium_rendering::{Renderer};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoTypes, VulkanoWindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderTargetRaw};

use {VkVertex, VulkanoSimple2DTypes, VulkanoSimple2DRenderer};

pub struct VulkanoSimple2DRenderTargetRaw {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    should_clear: bool,
}

impl VulkanoSimple2DRenderTargetRaw {
    pub fn pipeline(&self) -> &Arc<GraphicsPipelineAbstract + Send + Sync> {
        &self.pipeline
    }

    pub fn clear_values(&self) -> Vec<ClearValue> {
        if self.should_clear {
            vec!(ClearValue::Float([0.0, 0.0, 0.0, 1.0]))
        } else {
            vec!(ClearValue::None)
        }
    }
}

impl Simple2DRenderTargetRaw<VulkanoTypes, VulkanoSimple2DTypes> for VulkanoSimple2DRenderTargetRaw {
    fn new(
        should_clear: bool,
        renderer: &VulkanoRenderer, _window_renderer: &VulkanoWindowRenderer,
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

            .render_pass(Subpass::from(render_pass, 0).unwrap())
            .build(renderer.device().clone()).unwrap()
        ) as Arc<GraphicsPipeline<SingleBufferDefinition<VkVertex>, _, _>>;

        VulkanoSimple2DRenderTargetRaw {
            pipeline,
            should_clear,
        }
    }
}
