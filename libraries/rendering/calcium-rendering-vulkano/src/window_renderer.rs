use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::swapchain::{Surface};
use vulkano::sync::{GpuFuture};
use vulkano::framebuffer::{FramebufferAbstract};

use calcium_rendering::{WindowRenderer};
use {WindowSwapchain, VulkanoTypes, VulkanoRenderer};

pub struct VulkanoWindowRenderer {
    pub size: Vector2<u32>,
    pub surface: Arc<Surface>,
    pub swapchain: WindowSwapchain,
    queued_resize: Option<Vector2<u32>>,
}

impl VulkanoWindowRenderer {
    pub fn new(
        renderer: &VulkanoRenderer, surface: Arc<Surface>, size: Vector2<u32>,
    ) -> Self {
        info!(renderer.log, "Creating window renderer");

        // Create the swapchain we'll have to render to to make things actually show up on screen
        let swapchain = WindowSwapchain::new(renderer, &surface, size);

        VulkanoWindowRenderer {
            size,
            surface,
            swapchain,
            queued_resize: None,
        }
    }

    pub fn queue_resize(&mut self, size: Vector2<u32>) {
        // We can be spammed with resize events many times in the same frame, so defer it
        self.queued_resize = Some(Vector2::new(
            if size.x > 0 {size.x} else {1},
            if size.y > 0 {size.y} else {1},
        ));
    }
}

impl WindowRenderer<VulkanoTypes> for VulkanoWindowRenderer {
    fn start_frame(&mut self, renderer: &mut VulkanoRenderer) -> VulkanoFrame {
        self.swapchain.cleanup_finished_frames();

        // Before we render, see if we need to execute a queued resize
        if let Some(size) = self.queued_resize.take() {
            self.size = size;
            self.swapchain.resize(renderer, self.size);
        }

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

    fn finish_frame(&mut self, renderer: &mut VulkanoRenderer, mut frame: VulkanoFrame) {
        self.swapchain.finish_frame(
            frame.future.take().unwrap(), renderer.graphics_queue.clone(), frame.image_num
        );
    }

    fn size(&self) -> Vector2<u32> {
        self.size
    }
}

pub struct VulkanoFrame {
    pub size: Vector2<u32>,
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    pub image_num: usize,
    pub future: Option<Box<GpuFuture + Send + Sync>>,
}
