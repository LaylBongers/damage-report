use std::sync::{Arc};

use vulkano::device::{Device, Queue};

use calcium_rendering::{FactoryBackend};
use {VulkanoBackendTypes, VulkanoRenderer};

pub struct VulkanoFactoryBackend {
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
}

impl FactoryBackend<VulkanoBackendTypes> for VulkanoFactoryBackend {
    fn new(renderer: &VulkanoRenderer) -> Self {
        VulkanoFactoryBackend {
            device: renderer.device.clone(),
            graphics_queue: renderer.graphics_queue.clone(),
        }
    }
}
