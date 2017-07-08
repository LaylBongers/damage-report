use std::sync::{Arc};
use std::iter;

use cgmath::{self, Vector2};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::depth_stencil::{DepthStencil, Compare};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{Viewport};
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use slog::{Logger};

use calcium_rendering_simple2d::{Simple2DRenderer, RenderCommands};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoWindowRenderer, VulkanoBackendTypes, VulkanoFrame};
use calcium_rendering_vulkano_shaders::{simple2d_vs, simple2d_fs};

use {VkVertex};

pub struct VulkanoSimple2DRenderer {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl VulkanoSimple2DRenderer {
    pub fn new(log: &Logger, renderer: &VulkanoRenderer, window: &VulkanoWindowRenderer) -> Self {
        let render_pass = create_render_pass(log, renderer, window);
        let pipeline = create_pipeline(log, renderer, window, render_pass);

        VulkanoSimple2DRenderer {
            pipeline,
        }
    }
}

impl Simple2DRenderer<VulkanoBackendTypes> for VulkanoSimple2DRenderer {
    fn render(&mut self, frame: &mut VulkanoFrame, commands: RenderCommands) {
        let proj = cgmath::ortho(
            0.0, frame.size.x as f32,
            frame.size.y as f32, 0.0,
            1.0, -1.0
        );

        // Create a big mesh of all the rectangles we got told to draw
        let mut vertices = Vec::new();
        for rect in commands.rectangles {
            let start: Vector2<f32> = rect.0.cast();
            let size: Vector2<f32> = rect.1.cast();
            vertices.push(VkVertex {
                v_position: [start.x, start.y],
            });
            vertices.push(VkVertex {
                v_position: [start.x, start.y + size.y],
            });
            vertices.push(VkVertex {
                v_position: [start.x + size.x, start.y],
            });
        }
    }
}

fn create_render_pass(
    log: &Logger, renderer: &VulkanoRenderer, window: &VulkanoWindowRenderer,
) -> Arc<RenderPassAbstract + Send + Sync> {
    debug!(log, "Creating g-buffer render pass");
    #[allow(dead_code)]
    let render_pass = Arc::new(single_pass_renderpass!(renderer.device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: window.swapchain.swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());

    render_pass
}

fn create_pipeline(
    log: &Logger, renderer: &VulkanoRenderer, window: &VulkanoWindowRenderer,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    // Load in the shaders
    debug!(log, "Creating simple2d shaders");
    let vs = simple2d_vs::Shader::load(renderer.device.clone()).unwrap();
    let fs = simple2d_fs::Shader::load(renderer.device.clone()).unwrap();

    // Set up the pipeline itself
    debug!(log, "Creating gbuffer pipeline");
    let dimensions = window.size;
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

        .render_pass(Subpass::from(render_pass, 0).unwrap())
        .build(renderer.device.clone()).unwrap()
    ) as Arc<GraphicsPipeline<SingleBufferDefinition<VkVertex>, _, _>>
}
