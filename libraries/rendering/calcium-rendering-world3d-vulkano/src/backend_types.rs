use calcium_rendering_vulkano::{VulkanoBackendTypes};
use calcium_rendering_world3d::{WorldBackendTypes};
use mesh::{VulkanoMeshBackend};
use {VulkanoWorldRenderer};

#[derive(Clone)]
pub struct VulkanoWorldBackendTypes;

impl WorldBackendTypes<VulkanoBackendTypes> for VulkanoWorldBackendTypes {
    type MeshBackend = VulkanoMeshBackend;

    type WorldRenderer = VulkanoWorldRenderer;
}
