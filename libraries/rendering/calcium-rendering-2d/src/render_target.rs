use calcium_rendering::{Renderer};
use calcium_rendering::raw::{RawAccess, RendererRaw};

use raw::{Renderer2DRaw, Renderer2DTargetRaw};
use {Renderer2D};

pub struct Renderer2DTarget<R: RendererRaw, SR: Renderer2DRaw<R>> {
    pub raw: SR::RenderTargetRaw,
}

impl<R: RendererRaw, SR: Renderer2DRaw<R>> Renderer2DTarget<R, SR> {
    pub fn new(
        clear: bool,
        renderer: &Renderer<R>,
        simple2d_renderer: &Renderer2D<R, SR>,
    ) -> Self {
        let raw = Renderer2DTargetRaw::new(
            clear,
            renderer, simple2d_renderer
        );

        Renderer2DTarget {
            raw,
        }
    }
}

impl<R: RendererRaw, SR: Renderer2DRaw<R>> RawAccess<SR::RenderTargetRaw>
    for Renderer2DTarget<R, SR>
{
    fn raw(&self) -> &SR::RenderTargetRaw { &self.raw }
    fn raw_mut(&mut self) -> &mut SR::RenderTargetRaw { &mut self.raw }
}
