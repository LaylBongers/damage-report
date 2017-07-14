use gfx::{Resources, Factory};

use calcium_rendering_simple2d::{Simple2DTypes};
use calcium_rendering_gfx::{GfxTypes};

use {GfxSimple2DRenderer};

#[derive(Clone)]
pub struct GfxSimple2DTypes;

impl<R: Resources, F: Factory<R> + 'static> Simple2DTypes<GfxTypes<R, F>> for GfxSimple2DTypes {
    type Renderer = GfxSimple2DRenderer;
}