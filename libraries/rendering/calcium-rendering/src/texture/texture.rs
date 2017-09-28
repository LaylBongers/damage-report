use std::path::{PathBuf};
use std::sync::{Arc};

use cgmath::{Vector2};

use {Error, Renderer};
use raw::{TextureRaw, RawAccess, RendererRaw};
use texture::{TextureSource, TextureStoreFormat, SampleMode, TextureBytes};

pub struct TextureBuilder<'a, R: RendererRaw> {
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

impl<'a, R: RendererRaw> TextureBuilder<'a, R> {
    fn new() -> Self {
        const BLACK_TEXTURE_1PX: &[u8] = &[0];

        TextureBuilder {
            source: TextureSource::Bytes {
                bytes: BLACK_TEXTURE_1PX.into(),
                size: Vector2::new(1, 1),
                color: false,
            },
            store_format: TextureStoreFormat::Srgb,
            generate_mipmaps: false,
            sample_mode: SampleMode::Linear,
            _r: ::std::marker::PhantomData,
        }
    }

    pub fn build(self, renderer: &mut Renderer<R>) -> Result<Arc<Texture<R>>, Error> {
        let raw = R::TextureRaw::new(self, renderer)?;
        Ok(Arc::new(Texture { raw }))
    }

    pub fn from_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        let path = path.into();
        self.source = TextureSource::File(path);
        self
    }

    pub fn from_bytes<B: Into<TextureBytes<'a>>>(
        mut self, bytes: B, size: Vector2<u32>, color: bool,
    ) -> Self {
        self.source = TextureSource::Bytes {
            bytes: bytes.into(),
            size,
            color,
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

pub struct Texture<R: RendererRaw> {
    raw: R::TextureRaw
}

impl<R: RendererRaw> Texture<R> {
    pub fn new() -> TextureBuilder<'static, R> {
        TextureBuilder::new()
    }
}

impl<R: RendererRaw> RawAccess<R::TextureRaw> for Texture<R> {
    fn raw(&self) -> &R::TextureRaw { &self.raw }
    fn raw_mut(&mut self) -> &mut R::TextureRaw { &mut self.raw }
}
