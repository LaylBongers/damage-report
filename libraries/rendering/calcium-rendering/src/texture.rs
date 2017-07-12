use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};

use {BackendTypes, Error};

pub trait Texture<T: BackendTypes> {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    fn from_file(
        renderer: &mut T::Renderer, path: PathBuf, format: TextureFormat,
    ) -> Result<Arc<Self>, Error>;

    fn from_raw_greyscale(
        renderer: &mut T::Renderer, data: &[u8], size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error>;
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
