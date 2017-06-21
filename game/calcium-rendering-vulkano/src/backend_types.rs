use calcium_rendering::{BackendTypes};
use {VulkanoRenderBackend, VulkanoTextureBackend, VulkanoFrame, VulkanoFactoryBackend};

#[derive(Clone)]
pub struct VulkanoBackendTypes;

impl BackendTypes for VulkanoBackendTypes {
    type RenderBackend = VulkanoRenderBackend;
    type FactoryBackend = VulkanoFactoryBackend;
    type TextureBackend = VulkanoTextureBackend;

    type Frame = VulkanoFrame;
}
