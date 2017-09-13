use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::swapchain::{Surface};
use vulkano::sync::{GpuFuture};

use calcium_rendering::{WindowRenderer, Renderer};
use {WindowSwapchain, VulkanoRenderer};

pub struct VulkanoWindowRenderer {
    size: Vector2<u32>,
    pub surface: Arc<Surface>,
    pub swapchain: WindowSwapchain,
    queued_resize: bool,
    next_frame_id: u64,
}

impl VulkanoWindowRenderer {
    pub fn new(
        renderer: &VulkanoRenderer, surface: Arc<Surface>, size: Vector2<u32>,
    ) -> Self {
        info!(renderer.log(), "Creating window renderer");

        // Create the swapchain we'll have to render to to make things actually show up on screen
        let swapchain = WindowSwapchain::new(renderer, &surface, size);

        VulkanoWindowRenderer {
            size,
            surface,
            swapchain,
            queued_resize: false,
            next_frame_id: 0,
        }
    }

    pub fn queue_resize(&mut self, size: Vector2<u32>) {
        // Limit to at least 1x1 in size, we crash otherwise.
        if size.x <= 0 || size.y <= 0 {
            return
        }

        // We can be spammed with resize events many times in the same frame, so defer changing the
        //  swapchain.
        self.queued_resize = true;

        // We do however want to immediately set the size value as it may be used for 2D geometry
        // location calculations, which would lag behind at least one frame like this if the
        // calculations are done before start_frame.
        self.size = size;
    }
}

impl WindowRenderer<VulkanoRenderer> for VulkanoWindowRenderer {
    fn start_frame(&mut self, renderer: &mut VulkanoRenderer) -> VulkanoFrame {
        self.swapchain.cleanup_finished_frames();

        // Before we render, see if we need to execute a queued resize
        if self.queued_resize {
            // Overwrite the size with the actual size we were changed to
            self.size = self.swapchain.resize(self.size, renderer, &self.surface);
            self.queued_resize = false;
        }

        // Get the image for this frame, along with a future that will let us queue up the order of
        //  command buffer submissions.
        let (image_num, future) = self.swapchain.start_frame();

        self.next_frame_id += 1;
        VulkanoFrame {
            image_num,
            future: Some(future),
            frame_id: self.next_frame_id - 1,
        }
    }

    fn finish_frame(&mut self, renderer: &mut VulkanoRenderer, mut frame: VulkanoFrame) {
        self.swapchain.finish_frame(
            frame.future.take().unwrap(), renderer.graphics_queue().clone(), frame.image_num
        );
    }

    fn size(&self) -> Vector2<u32> {
        self.size
    }
}

pub struct VulkanoFrame {
    pub image_num: usize,
    pub future: Option<Box<GpuFuture + Send + Sync>>,
    pub frame_id: u64,
}
