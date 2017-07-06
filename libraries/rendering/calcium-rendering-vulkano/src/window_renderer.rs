use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::swapchain::{Surface};
use vulkano::instance::{PhysicalDevice};
use vulkano::device::{Device, Queue};
use slog::{Logger};

use {WindowSwapchain};

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
