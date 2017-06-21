use std::any::{Any};
use {RenderBackend, TextureBackend};

pub trait BackendTypes: Sized + Clone + Any {
    type RenderBackend: RenderBackend<Self>;
    type TextureBackend: TextureBackend<Self>;

    type Frame;
}
