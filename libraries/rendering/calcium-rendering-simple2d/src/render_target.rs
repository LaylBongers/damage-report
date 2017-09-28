use calcium_rendering::{Renderer};
use calcium_rendering::raw::{RawAccess, RendererRaw};

use raw::{Simple2DRendererRaw, Simple2DRenderTargetRaw};

pub struct Simple2DRenderTarget<R: RendererRaw, SR: Simple2DRendererRaw<R>> {
    pub raw: SR::RenderTargetRaw,
}

impl<R: RendererRaw, SR: Simple2DRendererRaw<R>> Simple2DRenderTarget<R, SR> {
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

impl<R: RendererRaw, SR: Simple2DRendererRaw<R>> RawAccess<SR::RenderTargetRaw>
    for Simple2DRenderTarget<R, SR>
{
    fn raw(&self) -> &SR::RenderTargetRaw { &self.raw }
    fn raw_mut(&mut self) -> &mut SR::RenderTargetRaw { &mut self.raw }
}
