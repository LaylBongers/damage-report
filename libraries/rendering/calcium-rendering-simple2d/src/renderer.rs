use calcium_rendering::{Types};
use {RenderBatch, Simple2DRenderTarget, Simple2DTypes};

/// A 2D renderer capable of rendering render batches.
pub trait Simple2DRenderer<T: Types, ST: Simple2DTypes<T>> {
    /// Renders the given render batches to a frame.
    fn render(
        &mut self,
        batches: &[RenderBatch<T>],
        render_target: &mut Simple2DRenderTarget<T, ST>,
        renderer: &mut T::Renderer,
        frame: &mut T::Frame,
    );
}
