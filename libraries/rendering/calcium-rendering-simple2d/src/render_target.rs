use calcium_rendering::{Renderer};
use {Simple2DRenderer};

pub struct Simple2DRenderTarget<R: Renderer, SR: Simple2DRenderer<R>> {
    pub raw: SR::RenderTargetRaw,
}

impl<R: Renderer, SR: Simple2DRenderer<R>> Simple2DRenderTarget<R, SR> {
    pub fn new(
        should_clear: bool,
        renderer: &R, window_renderer: &R::WindowRenderer,
        simple2d_renderer: &SR,
    ) -> Self {
        let raw = Simple2DRenderTargetRaw::new(
            should_clear, renderer, window_renderer, simple2d_renderer
        );

        Simple2DRenderTarget {
            raw,
        }
    }
}

pub trait Simple2DRenderTargetRaw<R: Renderer, SR: Simple2DRenderer<R>> {
    fn new(
        should_clear: bool,
        renderer: &R, window_renderer: &R::WindowRenderer,
        simple2d_renderer: &SR,
    ) -> Self;
}
