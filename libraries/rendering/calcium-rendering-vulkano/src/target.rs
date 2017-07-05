use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface};

pub trait VulkanoTargetSystem {
    fn required_extensions(&self) -> InstanceExtensions;
    fn create_surface(&mut self, instance: Arc<Instance>, size: Vector2<u32>) -> Arc<Surface>;
}
