use std::path::{PathBuf};
use std::sync::{Arc};

pub struct Texture {
    // Texture needs to track the data to load a texture internally because the backend may need to
    // be re-loaded, in which case the backend data gets purged.
    pub source: PathBuf,
    pub format: TextureFormat,
}

impl Texture {
    pub fn new<P: Into<PathBuf>>(path: P, format: TextureFormat) -> Arc<Self> {
        Arc::new(Texture {
            source: path.into(),
            format,
        })
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextureFormat {
    Srgb,
    Linear,
    LinearRed,
}
