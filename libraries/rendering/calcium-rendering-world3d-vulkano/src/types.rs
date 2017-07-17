use calcium_rendering_vulkano::{VulkanoTypes};
use calcium_rendering_world3d::{World3DTypes};
use mesh::{VulkanoMesh};

use {VulkanoWorld3DRenderer, VulkanoWorld3DRenderTargetRaw};

#[derive(Clone)]
pub struct VulkanoWorld3DTypes;

impl World3DTypes<VulkanoTypes> for VulkanoWorld3DTypes {
    type Renderer = VulkanoWorld3DRenderer;
    type RenderTargetRaw = VulkanoWorld3DRenderTargetRaw;
    type Mesh = VulkanoMesh;
}
