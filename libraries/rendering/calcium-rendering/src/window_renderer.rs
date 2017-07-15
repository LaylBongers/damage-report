use cgmath::{Vector2};
use {Types};

/// A representation of the render data needed to render to a window.
pub trait WindowRenderer<T: Types> {
    fn start_frame(&mut self, renderer: &mut T::Renderer) -> T::Frame;
    fn finish_frame(&mut self, renderer: &mut T::Renderer, frame: T::Frame);

    fn size(&self) -> Vector2<u32>;
}
