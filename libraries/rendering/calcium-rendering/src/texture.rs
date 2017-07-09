use std::path::{PathBuf};
use std::sync::{Arc};

use slog::{Logger};

use {BackendTypes};

pub trait Texture<T: BackendTypes> {
    fn load_file(
        log: &Logger, renderer: &mut T::Renderer, path: PathBuf, format: TextureFormat
    ) -> Arc<Self>;
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextureFormat {
    Srgb,
    Linear,
    LinearRed,
}
