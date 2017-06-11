use calcium_rendering::{RenderBackend};

pub struct VulkanoRenderBackend {
}

impl VulkanoRenderBackend {
    pub fn new() -> Box<RenderBackend> {
        Box::new(VulkanoRenderBackend {
        })
    }
}

impl RenderBackend for VulkanoRenderBackend {
}
