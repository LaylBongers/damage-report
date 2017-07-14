use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2};
use gfx::{Device, Factory};

use calcium_rendering::{Texture, TextureFormat, Error};

use {GfxTypes, GfxRenderer};

pub struct GfxTexture {
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Texture<GfxTypes<D, F>> for GfxTexture {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    fn from_file(
        _renderer: &mut GfxRenderer<D, F>, _path: PathBuf, _format: TextureFormat,
    ) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(GfxTexture {}))
    }

    fn from_raw_greyscale(
        _renderer: &mut GfxRenderer<D, F>, _data: &[u8], _size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(GfxTexture {}))
    }
}
