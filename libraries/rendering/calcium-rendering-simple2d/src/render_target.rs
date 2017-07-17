use calcium_rendering::{Types};
use {Simple2DTypes};

pub struct Simple2DRenderTarget<T: Types, ST: Simple2DTypes<T>> {
    pub raw: ST::RenderTargetRaw,
}

impl<T: Types, ST: Simple2DTypes<T>> Simple2DRenderTarget<T, ST> {
    pub fn new(
        should_clear: bool,
        renderer: &T::Renderer, window_renderer: &T::WindowRenderer,
        simple2d_renderer: &ST::Renderer,
    ) -> Self {
        let raw = Simple2DRenderTargetRaw::new(
            should_clear, renderer, window_renderer, simple2d_renderer
        );

        Simple2DRenderTarget {
            raw,
        }
    }
}

pub trait Simple2DRenderTargetRaw<T: Types, ST: Simple2DTypes<T>> {
    fn new(
        should_clear: bool,
        renderer: &T::Renderer, window_renderer: &T::WindowRenderer,
        simple2d_renderer: &ST::Renderer,
    ) -> Self;
}
