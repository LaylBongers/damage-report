use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::swapchain::{Surface};
use vulkano::instance::{PhysicalDevice};
use vulkano::device::{Device, Queue};
use vulkano::sync::{GpuFuture};
use vulkano::framebuffer::{FramebufferAbstract};
use slog::{Logger};

use calcium_rendering::{WindowRenderer};
use {WindowSwapchain, VulkanoBackendTypes, VulkanoRenderer};

pub struct VulkanoWindowRenderer {
    pub size: Vector2<u32>,
    pub surface: Arc<Surface>,
    pub swapchain: Option<WindowSwapchain>,
}

impl VulkanoWindowRenderer {
    pub fn new(surface: Arc<Surface>, size: Vector2<u32>) -> Self {
        VulkanoWindowRenderer {
            size,
            surface,
            swapchain: None,
        }
    }

    pub fn finish_initialization(
        &mut self,
        log: &Logger, physical: PhysicalDevice,
        device: Arc<Device>, graphics_queue: &Arc<Queue>
    ) {
        // Create the swapchain we'll have to render to to make things actually show up on screen
        let swapchain = WindowSwapchain::new(
            log, &self.surface, self.size, physical,
            device, graphics_queue,
        );

        self.swapchain = Some(swapchain);
    }
}

impl WindowRenderer<VulkanoBackendTypes> for VulkanoWindowRenderer {
    fn start_frame(&mut self) -> VulkanoFrame {
        self.swapchain.as_mut().unwrap().clean_old_submissions();

        // Get the image for this frame, along with a future that will let us queue up the order of
        //  command buffer submissions.
        let (framebuffer, image_num, future) = self.swapchain.as_ref().unwrap().start_frame();

        VulkanoFrame {
            framebuffer,
            image_num,
            future: Some(future),
        }
    }

    fn finish_frame(&mut self, renderer: &VulkanoRenderer, mut frame: VulkanoFrame) {
        self.swapchain.as_mut().unwrap().finish_frame(
            frame.future.take().unwrap(), renderer.graphics_queue.clone(), frame.image_num
        );
    }
}

pub struct VulkanoFrame {
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    pub image_num: usize,
    pub future: Option<Box<GpuFuture + Send + Sync>>,
}
