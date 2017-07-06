use std::path::{PathBuf};
use std::sync::{Arc};

use slog::{Logger};

use {BackendTypes};

pub struct Texture<T: BackendTypes> {
    pub backend: T::TextureBackend,
}

impl<T: BackendTypes> Texture<T> {
    pub fn new<P: Into<PathBuf>>(
        log: &Logger, renderer: &mut T::Renderer, path: P, format: TextureFormat
    ) -> Arc<Self> {
        let backend = T::TextureBackend::new(log, renderer, path.into(), format);

        Arc::new(Texture {
            backend,
        })
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextureFormat {
    Srgb,
    Linear,
    LinearRed,
}

pub trait TextureBackend<T: BackendTypes> {
    fn new(
        log: &Logger, renderer: &mut T::Renderer, path: PathBuf, format: TextureFormat
    ) -> Self;
}
