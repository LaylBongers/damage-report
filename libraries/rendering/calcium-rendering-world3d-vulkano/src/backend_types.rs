use calcium_rendering_vulkano::{VulkanoBackendTypes};
use calcium_rendering_world3d::{World3DBackendTypes};
use mesh::{VulkanoMeshBackend};
use {VulkanoWorld3DRenderer};

#[derive(Clone)]
pub struct VulkanoWorldBackendTypes;

impl World3DBackendTypes<VulkanoBackendTypes> for VulkanoWorldBackendTypes {
    type Renderer = VulkanoWorld3DRenderer;

    type Mesh = VulkanoMeshBackend;
}
