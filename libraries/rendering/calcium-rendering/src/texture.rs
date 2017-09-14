use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};

use {Renderer, Error};

pub struct TextureBuilder<'a, R: Renderer> {
    /// Where to get the pixel data for this texture from. Defaults to a 1px black texture.
    pub source: TextureSource<'a>,

    /// Defines how the texture should be stored internally. Defaults to Srgb.
    pub store_format: TextureStoreFormat,

    /// If set to true, mipmaps will be generated and used for this texture.
    pub generate_mipmaps: bool,

    /// How this texture should be sampled, mipmapping will be applied on top of this if applicable.
    pub sample_mode: SampleMode,

    _r: ::std::marker::PhantomData<R>,
}

impl<'a, R: Renderer> TextureBuilder<'a, R> {
    fn new() -> Self {
        const BLACK_TEXTURE_1PX: &[u8] = &[0];

        TextureBuilder {
            source: TextureSource::GreyscaleBytes {
                bytes: BLACK_TEXTURE_1PX.into(),
                size: Vector2::new(1, 1),
            },
            store_format: TextureStoreFormat::Srgb,
            generate_mipmaps: false,
            sample_mode: SampleMode::Linear,
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

    pub fn from_greyscale_bytes<B: Into<TextureBytes<'a>>>(
        mut self, bytes: B, size: Vector2<u32>
    ) -> Self {
        self.source = TextureSource::GreyscaleBytes {
            bytes: bytes.into(),
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

    pub fn generate_mipmaps(mut self) -> Self {
        self.generate_mipmaps = true;
        self
    }

    pub fn with_sample_mode(mut self, value: SampleMode) -> Self {
        self.sample_mode = value;
        self
    }

    pub fn with_linear_sampling(self) -> Self {
        self.with_sample_mode(SampleMode::Linear)
    }

    pub fn with_nearest_sampling(self) -> Self {
        self.with_sample_mode(SampleMode::Nearest)
    }
}

pub enum TextureSource<'a> {
    File(PathBuf),
    // TODO: Add color bytes
    GreyscaleBytes { bytes: TextureBytes<'a>, size: Vector2<u32> },
}

pub enum TextureBytes<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

impl<'a> AsRef<[u8]> for TextureBytes<'a> {
    fn as_ref(&self) -> &[u8] {
        match *self {
            TextureBytes::Borrowed(bytes) => bytes,
            TextureBytes::Owned(ref bytes) => &bytes,
        }
    }
}

impl<'a> From<&'a [u8]> for TextureBytes<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        TextureBytes::Borrowed(bytes)
    }
}

impl From<Vec<u8>> for TextureBytes<'static> {
    fn from(bytes: Vec<u8>) -> Self {
        TextureBytes::Owned(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SampleMode {
    Linear,
    Nearest,
}
