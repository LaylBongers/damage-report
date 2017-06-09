use std::sync::{Arc};

use vulkano::framebuffer::{FramebufferAbstract};
use vulkano::sync::{GpuFuture};

pub struct Frame {
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    pub image_num: usize,
    pub future: Option<Box<GpuFuture>>,
}
