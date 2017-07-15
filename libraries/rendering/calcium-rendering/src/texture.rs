use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};

use {Types, Error};

pub struct Texture<T: Types> {
    pub raw: T::TextureRaw
}

impl<T: Types> Texture<T> {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    pub fn from_file<P: Into<PathBuf>>(
        renderer: &mut T::Renderer, path: P, format: TextureFormat,
    ) -> Result<Arc<Self>, Error> {
        let path = path.into();
        let raw = T::TextureRaw::from_file(renderer, path, format)?;
        Ok(Arc::new(Self { raw }))
    }

    pub fn from_raw_greyscale(
        renderer: &mut T::Renderer, data: &[u8], size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error> {
        let raw = T::TextureRaw::from_raw_greyscale(renderer, data, size)?;
        Ok(Arc::new(Self { raw }))
    }
}

pub trait TextureRaw<T: Types>: Sized {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    fn from_file(
        renderer: &mut T::Renderer, path: PathBuf, format: TextureFormat,
    ) -> Result<Self, Error>;

    fn from_raw_greyscale(
        renderer: &mut T::Renderer, data: &[u8], size: Vector2<u32>,
    ) -> Result<Self, Error>;
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
