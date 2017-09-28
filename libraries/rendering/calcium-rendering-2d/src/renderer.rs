use calcium_rendering::raw::{RawAccess, RendererRaw};
use calcium_rendering::{Renderer, Frame};

use raw::{Renderer2DRaw};
use render_data::{RenderData};
use {Renderer2DTarget};

/// A 2D renderer capable of rendering render batches.
pub struct Renderer2D<R: RendererRaw, SR: Renderer2DRaw<R>> {
    raw: SR,
    _r: ::std::marker::PhantomData<R>,
}

impl<R: RendererRaw, SR: Renderer2DRaw<R>> Renderer2D<R, SR> {
    pub fn raw_new(raw: SR) -> Self {
        Renderer2D {
            raw,
            _r: Default::default(),
        }
    }

    /// Renders a collection of rendering data.
    pub fn render(
        &mut self,
        data: &RenderData<R>,
        frame: &mut Frame<R>,
        render_target: &mut Renderer2DTarget<R, SR>,
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

impl<R: RendererRaw, SR: Renderer2DRaw<R>> RawAccess<SR>
    for Renderer2D<R, SR>
{
    fn raw(&self) -> &SR { &self.raw }
    fn raw_mut(&mut self) -> &mut SR { &mut self.raw }
}
