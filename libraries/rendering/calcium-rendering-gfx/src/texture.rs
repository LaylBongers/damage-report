use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2};
use gfx::{Device, Factory};
use gfx::texture::{Kind, Size, AaMode};
use gfx::format::{Rgba8};
use gfx::handle::{ShaderResourceView};

use calcium_rendering::{Texture, TextureFormat, Error, CalciumErrorMappable};

use {GfxTypes, GfxRenderer};

pub struct GfxTexture<D: Device + 'static> {
    pub view: ShaderResourceView<D::Resources, [f32; 4]>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Texture<GfxTypes<D, F>> for GfxTexture<D> {
    // TODO: This is better suited to be handled by a separate asset manager crate.
    fn from_file(
        _renderer: &mut GfxRenderer<D, F>, _path: PathBuf, _format: TextureFormat,
    ) -> Result<Arc<Self>, Error> {
        unimplemented!()
    }

    fn from_raw_greyscale(
        renderer: &mut GfxRenderer<D, F>, data: &[u8], size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error> {
        // Create image data in RGBA format rather than the R format we got
        // TODO: Figure out a way to avoid this step
        let mut rgba = Vec::new();
        for r in data {
            rgba.push(*r);
            rgba.push(0);
            rgba.push(0);
            rgba.push(1);
        }

        // Actually create the gfx texture
        let kind = Kind::D2(size.x as Size, size.y as Size, AaMode::Single);
        let (_, view) = renderer.factory.create_texture_immutable_u8::<Rgba8>(kind, &[&rgba])
            .map_platform_err()?;

        Ok(Arc::new(GfxTexture {
            view
        }))
    }
}
