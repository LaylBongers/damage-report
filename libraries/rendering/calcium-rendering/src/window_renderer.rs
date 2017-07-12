use cgmath::{Vector2};
use {Types};

pub trait WindowRenderer<T: Types> {
    fn start_frame(&mut self, renderer: &T::Renderer) -> T::Frame;
    fn finish_frame(&mut self, renderer: &T::Renderer, frame: T::Frame);

    fn size(&self) -> Vector2<u32>;
}
