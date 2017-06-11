use calcium_rendering_world3d::{WorldRenderBackend};

pub struct VulkanoWorldRenderBackend {
}

impl VulkanoWorldRenderBackend {
    pub fn new() -> Box<WorldRenderBackend> {
        Box::new(VulkanoWorldRenderBackend {
        })
    }
}

impl WorldRenderBackend for VulkanoWorldRenderBackend {
}
