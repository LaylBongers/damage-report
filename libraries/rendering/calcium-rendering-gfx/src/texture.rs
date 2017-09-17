use std::path::{PathBuf};

use cgmath::{Vector2};
use image::{self};
use gfx::{Device, Factory};
use gfx::texture::{Kind, Size, AaMode};
use gfx::format::{Rgba8, Srgba8};
use gfx::handle::{ShaderResourceView};

use calcium_rendering::{Error, CalciumErrorMappable};
use calcium_rendering::texture::{TextureRaw, TextureBuilder, TextureSource, TextureStoreFormat, SampleMode};

use {GfxRenderer};

pub struct GfxTextureRaw<D: Device + 'static> {
    pub view: ShaderResourceView<D::Resources, [f32; 4]>,
    pub sample_mode: SampleMode,
}

impl<D: Device + 'static> GfxTextureRaw<D> {
    fn from_path<F: Factory<D::Resources> + 'static>(
        path: &PathBuf, builder: &TextureBuilder<GfxRenderer<D, F>>, renderer: &mut GfxRenderer<D, F>
    ) -> Result<Self, Error> {
        info!(renderer.log,
            "Loading texture from file"; "path" => path.display().to_string()
        );

        // Load in the image file
        let img = image::open(path).unwrap().to_rgba();;
        let (width, height) = img.dimensions();

        // TODO: Figure out a way to support single-channel formats, I don't think using the
        // following in place of Srgba8/Rgba8 will work by itself and I don't currently have time
        // to test it.
        //type Sr8 = (R8, Srgb);

        // Load in the texture
        let kind = Kind::D2(width as Size, height as Size, AaMode::Single);
        let (_, view) = match builder.store_format {
            TextureStoreFormat::Srgb =>
                renderer.factory.create_texture_immutable_u8::<Srgba8>(kind, &[&img]),
            TextureStoreFormat::Linear =>
                renderer.factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]),
            TextureStoreFormat::SingleChannel =>
                // TODO: Just show a warning and fall back to multi-channel linear
                panic!("GFX backend does not support converting multi-channel to greyscale"),
        }.map_platform_err()?;

        Ok(GfxTextureRaw {
            view,
            sample_mode: builder.sample_mode,
        })
    }

    fn from_bytes<F: Factory<D::Resources> + 'static>(
        bytes: &[u8], size: Vector2<u32>, color: bool,
        builder: &TextureBuilder<GfxRenderer<D, F>>, renderer: &mut GfxRenderer<D, F>,
    ) -> Result<Self, Error> {
        info!(renderer.log,
            "Loading texture from greyscale data"; "width" => size.x, "height" => size.y
        );

        // TODO: Support these other options
        if !color && builder.store_format != TextureStoreFormat::SingleChannel {
            panic!("GFX backend does not support converting greyscale to multi-channel");
        }
        if color && builder.store_format == TextureStoreFormat::SingleChannel {
            panic!("GFX backend does not support converting greyscale to multi-channel");
        }

        // Create image data in RGBA format rather than the R format we got
        // TODO: Avoid this step using the following advice:
        //  "create an Upload type buffer, map it, fill it up, then issue copy_buffer_to_texture"
        let mut rgba = vec![4; bytes.len() * 4];
        for i in 0..bytes.len() {
            unsafe {
                *rgba.get_unchecked_mut(i*4 + 0) = bytes[i];
                *rgba.get_unchecked_mut(i*4 + 1) = 0;
                *rgba.get_unchecked_mut(i*4 + 2) = 0;
                *rgba.get_unchecked_mut(i*4 + 3) = 1;
            }
        }

        // Actually create the gfx texture
        let kind = Kind::D2(size.x as Size, size.y as Size, AaMode::Single);
        let (_, view) = renderer.factory.create_texture_immutable_u8::<Rgba8>(kind, &[&rgba])
            .map_platform_err()?;

        Ok(GfxTextureRaw {
            view,
            sample_mode: builder.sample_mode,
        })
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    TextureRaw<GfxRenderer<D, F>> for GfxTextureRaw<D> {

    fn new(
        builder: TextureBuilder<GfxRenderer<D, F>>, renderer: &mut GfxRenderer<D, F>
    ) -> Result<Self, Error> {
        match builder.source {
            TextureSource::File(ref path) =>
                Self::from_path(path, &builder, renderer),
            TextureSource::Bytes { ref bytes, size, color } => {
                Self::from_bytes(bytes.as_ref(), size, color, &builder, renderer)
            },
        }
    }
}
