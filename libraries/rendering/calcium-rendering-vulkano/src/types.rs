use calcium_rendering::{Types};
use {VulkanoTexture, VulkanoFrame, VulkanoWindowRenderer, VulkanoRenderer};

#[derive(Clone)]
pub struct VulkanoTypes;

impl Types for VulkanoTypes {
    type WindowRenderer = VulkanoWindowRenderer;
    type Renderer = VulkanoRenderer;
    type Frame = VulkanoFrame;

    type Texture = VulkanoTexture;
}
