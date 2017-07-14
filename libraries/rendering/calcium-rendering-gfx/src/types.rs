use gfx::{Resources, Factory};

use calcium_rendering::{Types};

use {GfxTexture, GfxFrame, GfxWindowRenderer, GfxRenderer};

#[derive(Clone)]
pub struct GfxTypes<R, F> {
    _r: ::std::marker::PhantomData<R>,
    _f: ::std::marker::PhantomData<F>,
}

impl<R: Resources, F: Factory<R> + 'static> Types for GfxTypes<R, F> {
    type Renderer = GfxRenderer<R, F>;
    type WindowRenderer = GfxWindowRenderer;
    type Frame = GfxFrame;

    type Texture = GfxTexture;
}
