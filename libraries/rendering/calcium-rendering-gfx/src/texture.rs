use std::path::{PathBuf};

use cgmath::{Vector2};
use image::{self};
use gfx::{Device, Factory};
use gfx::texture::{Kind, Size, AaMode};
use gfx::format::{Rgba8, Srgba8, R8, Unorm};
use gfx::handle::{ShaderResourceView};

use calcium_rendering::{Error, CalciumErrorMappable};
use calcium_rendering::texture::{TextureRaw, TextureBuilder, TextureSource, TextureStoreFormat, SampleMode};

use {GfxRenderer};

pub enum GenericView<D: Device + 'static> {
    Rgba8(ShaderResourceView<D::Resources, [f32; 4]>),
    R8(ShaderResourceView<D::Resources, f32>),
}

pub struct GfxTextureRaw<D: Device + 'static> {
    pub view: GenericView<D>,
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
        let view = match builder.store_format {
            TextureStoreFormat::Srgb => GenericView::Rgba8(
                renderer.factory.create_texture_immutable_u8::<Srgba8>(kind, &[&img])
                    .map_platform_err()?.1
            ),
            TextureStoreFormat::Linear => GenericView::Rgba8(
                renderer.factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img])
                    .map_platform_err()?.1
            ),
            TextureStoreFormat::SingleChannel =>
                // TODO: Just show a warning and fall back to multi-channel linear
                panic!("GFX backend does not support converting multi-channel to greyscale")
            ,
        };

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
            "Loading texture from bytes"; "width" => size.x, "height" => size.y, "color" => color
        );

        // Create image data in RGBA format rather than the R format we got
        // TODO: Avoid this step using the following advice:
        //  "create an Upload type buffer, map it, fill it up, then issue copy_buffer_to_texture"
        let data = if builder.store_format != TextureStoreFormat::SingleChannel {
            let mut data = vec![4; size.x as usize * size.y as usize * 4];
            if !color {
                // Multi-Channel store, Single-Channel source
                for i in 0..bytes.len() {
                    unsafe {
                        *data.get_unchecked_mut(i*4 + 0) = bytes[i];
                        *data.get_unchecked_mut(i*4 + 1) = 0;
                        *data.get_unchecked_mut(i*4 + 2) = 0;
                        *data.get_unchecked_mut(i*4 + 3) = 1;
                    }
                }
            } else {
                // Multi-Channel store, Multi-Channel source
                for i in 0..bytes.len() {
                    unsafe {
                        *data.get_unchecked_mut(i*4 + 0) = bytes[i*4 + 0];
                        *data.get_unchecked_mut(i*4 + 1) = bytes[i*4 + 1];
                        *data.get_unchecked_mut(i*4 + 2) = bytes[i*4 + 2];
                        *data.get_unchecked_mut(i*4 + 3) = bytes[i*4 + 3];
                    }
                }
            }
            data
        } else {
            let mut data = vec![4; size.x as usize * size.y as usize];
            if color { panic!("Currently unsupported conversion from color to greyscale in bytes"); }

            for i in 0..bytes.len() {
                unsafe {
                    *data.get_unchecked_mut(i) = bytes[i];
                }
            }

            data
        };

        // Additional shorthand types gfx doesn't have by itself
        type R8U = (R8, Unorm);

        // Actually create the gfx texture
        let kind = Kind::D2(size.x as Size, size.y as Size, AaMode::Single);
        let view = match builder.store_format {
            TextureStoreFormat::Srgb => GenericView::Rgba8(
                renderer.factory.create_texture_immutable_u8::<Srgba8>(kind, &[&data])
                    .map_platform_err()?.1
                ),
            TextureStoreFormat::Linear => GenericView::Rgba8(
                renderer.factory.create_texture_immutable_u8::<Rgba8>(kind, &[&data])
                    .map_platform_err()?.1
                ),
            TextureStoreFormat::SingleChannel => GenericView::R8(
                renderer.factory.create_texture_immutable_u8::<R8U>(kind, &[&data])
                    .map_platform_err()?.1
                ),
        };

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
