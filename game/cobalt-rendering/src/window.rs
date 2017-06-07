use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface};

pub trait WindowCreator {
    type W: Window;

    fn required_extensions(&self) -> InstanceExtensions;
    fn create_window(&self, instance: Arc<Instance>, size: Vector2<u32>) -> Self::W;
}

pub trait Window {
    fn surface(&self) -> &Arc<Surface>;
}
