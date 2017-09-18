mod texture;

pub use self::texture::{TextureBuilder, Texture, TextureRaw};

use std::path::{PathBuf};

use cgmath::{Vector2};

pub enum TextureSource<'a> {
    File(PathBuf),
    Bytes { bytes: TextureBytes<'a>, size: Vector2<u32>, color: bool },
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SampleMode {
    Linear,
    Nearest,
}
