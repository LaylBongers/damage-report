use gfx::{Device, Factory};

use calcium_rendering::{Types};

use {GfxTexture, GfxFrame, GfxWindowRenderer, GfxRenderer};

#[derive(Clone)]
pub struct GfxTypes<D, F> {
    _d: ::std::marker::PhantomData<D>,
    _f: ::std::marker::PhantomData<F>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Types for GfxTypes<D, F> {
    type Renderer = GfxRenderer<D, F>;
    type WindowRenderer = GfxWindowRenderer;
    type Frame = GfxFrame;

    type Texture = GfxTexture<D>;
}
