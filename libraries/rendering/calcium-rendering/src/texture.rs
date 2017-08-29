use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};

use {Renderer, Error};

pub struct TextureBuilder<'a, R: Renderer> {
    /// Where to get the pixel data for this texture from. Defaults to a 1px black texture.
    pub source: TextureSource<'a>,

    /// Defines how the texture should be stored internally. Defaults to Srgb.
    pub store_format: TextureStoreFormat,

    _r: ::std::marker::PhantomData<R>,
}

impl<'a, R: Renderer> TextureBuilder<'a, R> {
    fn new() -> Self {
        const BLACK_TEXTURE_1PX: &[u8] = &[0];

        TextureBuilder {
            source: TextureSource::GreyscaleBytes {
                bytes: &BLACK_TEXTURE_1PX,
                size: Vector2::new(1, 1),
            },
            store_format: TextureStoreFormat::Srgb,
            _r: ::std::marker::PhantomData,
        }
    }

    pub fn build(self, renderer: &mut R) -> Result<Arc<Texture<R>>, Error> {
        let raw = R::TextureRaw::new(self, renderer)?;
        Ok(Arc::new(Texture { raw }))
    }

    pub fn from_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        let path = path.into();
        self.source = TextureSource::File(path);
        self
    }

    pub fn from_greyscale_bytes(mut self, bytes: &'a [u8], size: Vector2<u32>) -> Self {
        self.source = TextureSource::GreyscaleBytes {
            bytes,
            size,
        };
        self
    }

    pub fn with_store_format(mut self, value: TextureStoreFormat) -> Self {
        self.store_format = value;
        self
    }

    pub fn as_rgb(self) -> Self {
        self.with_store_format(TextureStoreFormat::Srgb)
    }

    pub fn as_linear(self) -> Self {
        self.with_store_format(TextureStoreFormat::Linear)
    }

    pub fn as_single_channel(self) -> Self {
        self.with_store_format(TextureStoreFormat::SingleChannel)
    }
}

pub enum TextureSource<'a> {
    File(PathBuf),
    // TODO: Add color bytes
    GreyscaleBytes { bytes: &'a [u8], size: Vector2<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextureStoreFormat {
    /// Interprets the texture as being in sRGB color space. This should be used for color data.
    /// The texture's pixels will be gamma converted to linear values in the pixel shaders.
    Srgb,

    /// Interprets the texture as being in linear color space. This should be used for non-color
    /// maps, like normal maps. The texture will be used as-is in the pixel shaders.
    Linear,

    /// Only a single color channel of this texture will be stored. This is useful for masks, and
    /// roughness/metallic/ambient occlusion maps. From textures with more than one channel the
    /// red channel will be used. This will be interpreted in linear color space.
    SingleChannel,
}

pub struct Texture<R: Renderer> {
    pub raw: R::TextureRaw
}

impl<R: Renderer> Texture<R> {
    pub fn new() -> TextureBuilder<'static, R> {
        TextureBuilder::new()
    }
}

pub trait TextureRaw<R: Renderer>: Sized {
    fn new(builder: TextureBuilder<R>, renderer: &mut R) -> Result<Self, Error>;
}
