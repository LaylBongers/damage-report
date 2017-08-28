use calcium_rendering::{Renderer};
use {Simple2DRenderer};

pub struct Simple2DRenderTarget<R: Renderer, SR: Simple2DRenderer<R>> {
    pub raw: SR::RenderTargetRaw,
}

impl<R: Renderer, SR: Simple2DRenderer<R>> Simple2DRenderTarget<R, SR> {
    pub fn new(
        // TODO: This is needed because the vulkano backend needs a different renderpass when clear
        // is enabled or disabled. Find a way to avoid having to create an expensive render target
        // just to decide if it should clear or not.
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
