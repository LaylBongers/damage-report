use calcium_rendering::{BackendTypes};
use {VulkanoRenderBackend, VulkanoTextureBackend, VulkanoFrame};

#[derive(Clone)]
pub struct VulkanoBackendTypes;

impl BackendTypes for VulkanoBackendTypes {
    type RenderBackend = VulkanoRenderBackend;
    type TextureBackend = VulkanoTextureBackend;

    type Frame = VulkanoFrame;
}
