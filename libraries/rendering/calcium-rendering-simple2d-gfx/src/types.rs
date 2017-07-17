use gfx::{Device, Factory};

use calcium_rendering_simple2d::{Simple2DTypes};
use calcium_rendering_gfx::{GfxTypes};

use {GfxSimple2DRenderer, GfxSimple2DRenderTargetRaw};

#[derive(Clone)]
pub struct GfxSimple2DTypes;

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DTypes<GfxTypes<D, F>> for GfxSimple2DTypes {
    type Renderer = GfxSimple2DRenderer<D, F>;
    type RenderTargetRaw = GfxSimple2DRenderTargetRaw;
}
