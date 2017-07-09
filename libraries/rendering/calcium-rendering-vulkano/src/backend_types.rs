use calcium_rendering::{BackendTypes};
use {VulkanoTexture, VulkanoFrame, VulkanoWindowRenderer, VulkanoRenderer};

#[derive(Clone)]
pub struct VulkanoBackendTypes;

impl BackendTypes for VulkanoBackendTypes {
    type WindowRenderer = VulkanoWindowRenderer;
    type Renderer = VulkanoRenderer;
    type Frame = VulkanoFrame;

    type Texture = VulkanoTexture;
}
