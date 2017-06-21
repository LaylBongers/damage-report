use std::path::{PathBuf};
use std::sync::{Arc};

use slog::{Logger};

use {BackendTypes, RenderSystem};

pub struct Texture<T: BackendTypes> {
    pub backend: T::TextureBackend,
}

impl<T: BackendTypes> Texture<T> {
    pub fn new<P: Into<PathBuf>>(
        log: &Logger, render_system: &mut RenderSystem<T>, path: P, format: TextureFormat
    ) -> Arc<Self> {
        let backend = T::TextureBackend::load(log, &mut render_system.backend, path.into(), format);

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
    fn load(log: &Logger, backend: &mut T::RenderBackend, path: PathBuf, format: TextureFormat) -> Self;
}
