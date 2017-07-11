use calcium_rendering::{BackendTypes};
use {RenderBatch};

pub trait Simple2DRenderer<T: BackendTypes> {
    fn render(
        &mut self, renderer: &mut T::Renderer, frame: &mut T::Frame,
        batches: &Vec<RenderBatch<T>>
    );
}
