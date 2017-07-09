use cgmath::{Vector2};
use {BackendTypes};

pub trait WindowRenderer<T: BackendTypes> {
    fn start_frame(&mut self) -> T::Frame;
    fn finish_frame(&mut self, renderer: &T::Renderer, frame: T::Frame);

    fn size(&self) -> Vector2<u32>;
}
