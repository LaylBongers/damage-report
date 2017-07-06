use calcium_rendering::{BackendTypes};
use {VulkanoSystemContext, VulkanoTextureBackend, VulkanoFrame, VulkanoFactoryBackend, VulkanoWindowRenderer, VulkanoRenderer};

#[derive(Clone)]
pub struct VulkanoBackendTypes;

impl BackendTypes for VulkanoBackendTypes {
    type FactoryBackend = VulkanoFactoryBackend;
    type TextureBackend = VulkanoTextureBackend;

    type SystemContext = VulkanoSystemContext;
    type WindowRenderer = VulkanoWindowRenderer;
    type Renderer = VulkanoRenderer;
    type Frame = VulkanoFrame;
}
