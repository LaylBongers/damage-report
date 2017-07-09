use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};
use slog::{Logger};

use {BackendTypes};

pub trait Texture<T: BackendTypes> {
    fn from_file(
        log: &Logger, renderer: &mut T::Renderer, path: PathBuf, format: TextureFormat,
    ) -> Arc<Self>;

    fn from_raw_greyscale(
        log: &Logger, renderer: &mut T::Renderer, data: &[u8], size: Vector2<u32>,
    ) -> Arc<Self>;
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextureFormat {
    Srgb,
    Linear,
    LinearRed,
}
