use calcium_rendering::{Renderer};
use calcium_rendering::raw::{RawAccess, RendererRaw};

use {Simple2DRenderer};

pub struct Simple2DRenderTarget<R: RendererRaw, SR: Simple2DRenderer<R>> {
    pub raw: SR::RenderTargetRaw,
}

impl<R: RendererRaw, SR: Simple2DRenderer<R>> Simple2DRenderTarget<R, SR> {
    pub fn new(
        clear: bool,
        renderer: &Renderer<R>,
        simple2d_renderer: &SR,
    ) -> Self {
        let raw = Simple2DRenderTargetRaw::new(
            clear,
            renderer, simple2d_renderer
        );

        Simple2DRenderTarget {
            raw,
        }
    }
}

impl<R: RendererRaw, SR: Simple2DRenderer<R>> RawAccess<SR::RenderTargetRaw>
    for Simple2DRenderTarget<R, SR>
{
    fn raw(&self) -> &SR::RenderTargetRaw { &self.raw }
    fn raw_mut(&mut self) -> &mut SR::RenderTargetRaw { &mut self.raw }
}

pub trait Simple2DRenderTargetRaw<R: RendererRaw, SR: Simple2DRenderer<R>> {
    fn new(
        clear: bool,
        renderer: &Renderer<R>,
        simple2d_renderer: &SR,
    ) -> Self;
}
