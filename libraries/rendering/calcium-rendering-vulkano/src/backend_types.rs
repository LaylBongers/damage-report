use calcium_rendering::{BackendTypes};
use {VulkanoTextureBackend, VulkanoFrame, VulkanoFactoryBackend, VulkanoWindowRenderer, VulkanoRenderer};

#[derive(Clone)]
pub struct VulkanoBackendTypes;

impl BackendTypes for VulkanoBackendTypes {
    type FactoryBackend = VulkanoFactoryBackend;
    type TextureBackend = VulkanoTextureBackend;

    type WindowRenderer = VulkanoWindowRenderer;
    type Renderer = VulkanoRenderer;
    type Frame = VulkanoFrame;
}
