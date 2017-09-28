use calcium_rendering::raw::{RawAccess, RendererRaw};
use calcium_rendering::{Renderer, Frame};

use raw::{Simple2DRendererRaw};
use render_data::{RenderData};
use {Simple2DRenderTarget};

/// A 2D renderer capable of rendering render batches.
pub struct Simple2DRenderer<R: RendererRaw, SR: Simple2DRendererRaw<R>> {
    raw: SR,
    _r: ::std::marker::PhantomData<R>,
}

impl<R: RendererRaw, SR: Simple2DRendererRaw<R>> Simple2DRenderer<R, SR> {
    pub fn raw_new(raw: SR) -> Self {
        Simple2DRenderer {
            raw,
            _r: Default::default(),
        }
    }

    /// Renders a collection of rendering data.
    pub fn render(
        &mut self,
        data: &RenderData<R>,
        frame: &mut Frame<R>,
        render_target: &mut Simple2DRenderTarget<R, SR>,
        renderer: &mut Renderer<R>,
    ) {
        self.raw.render(
            data,
            frame,
            render_target,
            renderer,
        );
    }
}

impl<R: RendererRaw, SR: Simple2DRendererRaw<R>> RawAccess<SR>
    for Simple2DRenderer<R, SR>
{
    fn raw(&self) -> &SR { &self.raw }
    fn raw_mut(&mut self) -> &mut SR { &mut self.raw }
}
