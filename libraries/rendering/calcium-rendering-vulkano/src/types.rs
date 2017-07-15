use calcium_rendering::{Types};
use {VulkanoTextureRaw, VulkanoFrame, VulkanoWindowRenderer, VulkanoRenderer};

#[derive(Clone)]
pub struct VulkanoTypes;

impl Types for VulkanoTypes {
    type Renderer = VulkanoRenderer;
    type WindowRenderer = VulkanoWindowRenderer;
    type Frame = VulkanoFrame;

    type TextureRaw = VulkanoTextureRaw;
}
