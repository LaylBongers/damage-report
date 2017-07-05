use calcium_rendering_vulkano::{VulkanoBackendTypes};
use calcium_rendering_world3d::{WorldBackendTypes};
use mesh::{VulkanoMeshBackend};
use {VulkanoWorldRenderBackend};

#[derive(Clone)]
pub struct VulkanoWorldBackendTypes;

impl WorldBackendTypes<VulkanoBackendTypes> for VulkanoWorldBackendTypes {
    type WorldRenderBackend = VulkanoWorldRenderBackend;
    type MeshBackend = VulkanoMeshBackend;
}
