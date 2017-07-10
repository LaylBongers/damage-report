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

/// The format of raw texture data.
#[derive(PartialEq, Clone, Copy)]
pub enum TextureFormat {
    Srgb,
    Linear,
    // TODO: This is used as a special case to convert LinearRGB to single channel, there instead
    //  should be a separate enum for the target format the data should be changed to.
    LinearRed,
}
