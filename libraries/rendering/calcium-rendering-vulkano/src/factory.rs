use std::sync::{Arc};

use vulkano::device::{Device, Queue};

use calcium_rendering::{FactoryBackend};
use {VulkanoBackendTypes, VulkanoRenderBackend};

pub struct VulkanoFactoryBackend {
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
}

impl FactoryBackend<VulkanoBackendTypes> for VulkanoFactoryBackend {
    fn new(backend: &VulkanoRenderBackend) -> Self {
        VulkanoFactoryBackend {
            device: backend.device.clone(),
            graphics_queue: backend.graphics_queue.clone(),
        }
    }
}
