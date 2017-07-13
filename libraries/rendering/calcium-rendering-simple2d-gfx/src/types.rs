use calcium_rendering_simple2d::{Simple2DTypes};
use calcium_rendering_gfx::{GfxTypes};

use {GfxSimple2DRenderer};

#[derive(Clone)]
pub struct GfxSimple2DTypes;

impl Simple2DTypes<GfxTypes> for GfxSimple2DTypes {
    type Renderer = GfxSimple2DRenderer;
}
