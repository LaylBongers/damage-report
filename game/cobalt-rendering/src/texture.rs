use std::sync::{Arc};
use std::path::{Path};

use image::{self, GenericImage};
use slog::{Logger};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::image::{Dimensions};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};

use {Target};

/// An uploaded texture. Internally ref-counted, cheap clone.
#[derive(Clone)]
pub struct Texture {
    texture: Arc<ImmutableImage<Format>>,
    sampler: Arc<Sampler>,
}

impl Texture {
    pub fn load<P: AsRef<Path>>(
        log: &Logger, target: &mut Target, path: P, format: TextureFormat
    ) -> Self {
        // Load in the image file
        info!(log, "Loading texture"; "path" => path.as_ref().display().to_string());
        let img = image::open(path.as_ref()).unwrap();
        let img_dimensions = img.dimensions();

        // Load the image data into a buffer
        let buffer = {
            let image_data = img.raw_pixels();

            // If the format is LinearRed, we need to ignore the GBA elements
            let chunk_size = if format != TextureFormat::LinearRed { 1 } else { 4 };
            let image_data_iter = image_data.chunks(chunk_size).map(|c| c[0]);

            // TODO: staging buffer instead
            CpuAccessibleBuffer::<[u8]>::from_iter(
                target.device().clone(), BufferUsage::all(),
                Some(target.graphics_queue().family()), image_data_iter
            ).unwrap()
        };

        // Get the correct format for the srgb parameter we got passed
        let format = match format {
            TextureFormat::Srgb => Format::R8G8B8A8Srgb,
            TextureFormat::Linear => Format::R8G8B8A8Unorm,
            TextureFormat::LinearRed => Format::R8Unorm,
        };

        // Create the texture and sampler for the image, the texture data will later be copied in
        //  a command buffer
        let texture = ImmutableImage::new(
            target.device().clone(),
            Dimensions::Dim2d { width: img_dimensions.0, height: img_dimensions.1 },
            format, Some(target.graphics_queue().family())
        ).unwrap();
        let sampler = Sampler::new(
            target.device().clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).unwrap();

        // Make sure the buffer's actually put into the texture
        target.queue_texture_copy(buffer, texture.clone());

        Texture {
            texture,
            sampler,
        }
    }

    pub fn uniform(&self) -> (Arc<ImmutableImage<Format>>, Arc<Sampler>) {
        (self.texture.clone(), self.sampler.clone())
    }
}

#[derive(PartialEq)]
pub enum TextureFormat {
    Srgb,
    Linear,
    LinearRed,
}
