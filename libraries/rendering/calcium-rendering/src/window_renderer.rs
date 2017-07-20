use cgmath::{Vector2};
use {Renderer};

/// A representation of the render data needed to render to a window.
pub trait WindowRenderer<R: Renderer> {
    fn start_frame(&mut self, renderer: &mut R) -> R::Frame;
    fn finish_frame(&mut self, renderer: &mut R, frame: R::Frame);

    fn size(&self) -> Vector2<u32>;
}
