use calcium_rendering::{BackendTypes};
use {RenderCommands};

pub trait Simple2DRenderer<T: BackendTypes> {
    fn render(&mut self, renderer: &T::Renderer, frame: &mut T::Frame, commands: RenderCommands);
}
