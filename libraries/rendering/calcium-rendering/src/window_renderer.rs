use cgmath::{Vector2};
use input::{Input};
use {Types};

/// A representation of the render data needed to render to a window.
pub trait WindowRenderer<T: Types> {
    /// Handles an event, updating the window as needed. Using this the backend can resize its back
    /// buffers and make other relevant changes.
    fn handle_event(&mut self, input: &Input);

    fn start_frame(&mut self, renderer: &mut T::Renderer) -> T::Frame;
    fn finish_frame(&mut self, renderer: &mut T::Renderer, frame: T::Frame);

    fn size(&self) -> Vector2<u32>;
}
