use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2};
use gfx::{Resources, Factory};

use calcium_rendering::{Texture, TextureFormat, Error};

use {GfxTypes, GfxRenderer};

pub struct GfxTexture {
}

impl<R: Resources, F: Factory<R> + 'static> Texture<GfxTypes<R, F>> for GfxTexture {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    fn from_file(
        _renderer: &mut GfxRenderer<R, F>, _path: PathBuf, _format: TextureFormat,
    ) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(GfxTexture {}))
    }

    fn from_raw_greyscale(
        _renderer: &mut GfxRenderer<R, F>, _data: &[u8], _size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(GfxTexture {}))
    }
}
