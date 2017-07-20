use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};

use {Renderer, Error};

pub struct Texture<R: Renderer> {
    pub raw: R::TextureRaw
}

impl<R: Renderer> Texture<R> {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    pub fn from_file<P: Into<PathBuf>>(
        renderer: &mut R, path: P, format: TextureFormat,
    ) -> Result<Arc<Self>, Error> {
        let path = path.into();
        let raw = R::TextureRaw::from_file(renderer, path, format)?;
        Ok(Arc::new(Self { raw }))
    }

    pub fn from_raw_greyscale(
        renderer: &mut R, data: &[u8], size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error> {
        let raw = R::TextureRaw::from_raw_greyscale(renderer, data, size)?;
        Ok(Arc::new(Self { raw }))
    }
}

pub trait TextureRaw<R: Renderer>: Sized {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    fn from_file(
        renderer: &mut R, path: PathBuf, format: TextureFormat,
    ) -> Result<Self, Error>;

    fn from_raw_greyscale(
        renderer: &mut R, data: &[u8], size: Vector2<u32>,
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
