use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer};
use vulkano::device::{Device, Queue};
use vulkano::image::immutable::{ImmutableImage};

use target_swapchain::{TargetSwapchain};
use {Frame};

pub trait Backend {
    fn start_frame(&mut self) -> Frame;
    fn finish_frame(&mut self, Frame);

    // TODO: These functions are implementation-specific, they're here right now while
    //  transitioning to a flexible backend system
    fn queue_texture_copy(
        &mut self,
        buffer: Arc<CpuAccessibleBuffer<[u8]>>,
        texture: Arc<ImmutableImage<Format>>,
    );
    fn device(&self) -> &Arc<Device>;
    fn graphics_queue(&self) -> &Arc<Queue>;
    fn swapchain(&self) -> &TargetSwapchain;
    fn size(&self) -> Vector2<u32>;
}
