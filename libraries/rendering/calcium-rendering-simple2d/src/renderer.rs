use calcium_rendering::{BackendTypes};
use {RenderBatch};

/// A 2D renderer capable of rendering render batches.
pub trait Simple2DRenderer<T: BackendTypes> {
    /// Renders the given render batches to a frame.
    fn render(
        &mut self, renderer: &mut T::Renderer, frame: &mut T::Frame,
        batches: &[RenderBatch<T>]
    );
}
