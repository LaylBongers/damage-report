extern crate calcium_rendering_simple2d;

use calcium_rendering_simple2d::{Simple2DBackendTypes};

pub struct VulkanoSimple2DRenderer {
}

impl VulkanoSimple2DRenderer {
    pub fn new() -> Self {
        VulkanoSimple2DRenderer {
        }
    }
}

#[derive(Clone)]
pub struct VulkanoSimple2DBackendTypes;

impl Simple2DBackendTypes for VulkanoSimple2DBackendTypes {
    type Renderer = VulkanoSimple2DRenderer;
}
