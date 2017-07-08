use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::swapchain::{Surface};
use vulkano::sync::{GpuFuture};
use vulkano::framebuffer::{FramebufferAbstract};
use slog::{Logger};

use calcium_rendering::{WindowRenderer};
use {WindowSwapchain, VulkanoBackendTypes, VulkanoRenderer};

pub struct VulkanoWindowRenderer {
    pub size: Vector2<u32>,
    pub surface: Arc<Surface>,
    pub swapchain: WindowSwapchain,
}

impl VulkanoWindowRenderer {
    pub fn new(
        log: &Logger, renderer: &VulkanoRenderer, surface: Arc<Surface>, size: Vector2<u32>,
    ) -> Self {
        info!(log, "Creating window renderer");

        // Create the swapchain we'll have to render to to make things actually show up on screen
        let swapchain = WindowSwapchain::new(log, renderer, &surface, size);

        VulkanoWindowRenderer {
            size,
            surface,
            swapchain,
        }
    }
}

impl WindowRenderer<VulkanoBackendTypes> for VulkanoWindowRenderer {
    fn start_frame(&mut self) -> VulkanoFrame {
        self.swapchain.cleanup_finished_frames();

        // Get the image for this frame, along with a future that will let us queue up the order of
        //  command buffer submissions.
        let (framebuffer, image_num, future) = self.swapchain.start_frame();

        VulkanoFrame {
            size: self.size,
            framebuffer,
            image_num,
            future: Some(future),
        }
    }

    fn finish_frame(&mut self, renderer: &VulkanoRenderer, mut frame: VulkanoFrame) {
        self.swapchain.finish_frame(
            frame.future.take().unwrap(), renderer.graphics_queue.clone(), frame.image_num
        );
    }
}

pub struct VulkanoFrame {
    pub size: Vector2<u32>,
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    pub image_num: usize,
    pub future: Option<Box<GpuFuture + Send + Sync>>,
}
