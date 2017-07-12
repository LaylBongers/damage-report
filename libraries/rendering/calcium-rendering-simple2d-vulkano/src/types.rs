use calcium_rendering_simple2d::{Simple2DTypes};
use calcium_rendering_vulkano::{VulkanoTypes};

use {VulkanoSimple2DRenderer};

#[derive(Clone)]
pub struct VulkanoSimple2DTypes;

impl Simple2DTypes<VulkanoTypes> for VulkanoSimple2DTypes {
    type Renderer = VulkanoSimple2DRenderer;
}
