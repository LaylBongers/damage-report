use calcium_rendering::{Types};
use {GfxTexture, GfxFrame, GfxWindowRenderer, GfxRenderer};

#[derive(Clone)]
pub struct GfxTypes;

impl Types for GfxTypes {
    type Renderer = GfxRenderer;
    type WindowRenderer = GfxWindowRenderer;
    type Frame = GfxFrame;

    type Texture = GfxTexture;
}
