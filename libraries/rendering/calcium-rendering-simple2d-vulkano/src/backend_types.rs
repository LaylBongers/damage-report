use calcium_rendering_simple2d::{Simple2DBackendTypes};
use calcium_rendering_vulkano::{VulkanoBackendTypes};

use {VulkanoSimple2DRenderer};

#[derive(Clone)]
pub struct VulkanoSimple2DBackendTypes;

impl Simple2DBackendTypes<VulkanoBackendTypes> for VulkanoSimple2DBackendTypes {
    type Renderer = VulkanoSimple2DRenderer;
}
