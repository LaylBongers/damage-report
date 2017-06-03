use std::sync::{Arc};
use std::path::{Path};

use image::{self, GenericImage};
use slog::{Logger};
use vulkano::format::{self, FormatDesc};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::image::{Dimensions};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};

use {Target};

// TODO: This file contains different texture variants depending on format, we need to find a way
// to merge their functionality better.

/// An uploaded texture. Internally ref-counted, cheap clone.
#[derive(Clone)]
pub struct TextureSrgb {
    texture: Arc<ImmutableImage<format::R8G8B8A8Srgb>>,
    sampler: Arc<Sampler>,
}

impl TextureSrgb {
    pub fn load<P: AsRef<Path>>(log: &Logger, target: &mut Target, path: P) -> Self {
        let (texture, sampler) = load_texture(log, target, path,
            || format::R8G8B8A8Srgb,
            |target, buffer, texture| target.queue_srgb_texture_copy(buffer, texture),
        );

        TextureSrgb {
            texture,
            sampler,
        }
    }

    pub fn uniform(&self) -> (Arc<ImmutableImage<format::R8G8B8A8Srgb>>, Arc<Sampler>) {
        (self.texture.clone(), self.sampler.clone())
    }
}

/// An uploaded texture. Internally ref-counted, cheap clone.
#[derive(Clone)]
pub struct TextureLinear {
    texture: Arc<ImmutableImage<format::R8G8B8A8Unorm>>,
    sampler: Arc<Sampler>,
}

impl TextureLinear {
    pub fn load<P: AsRef<Path>>(log: &Logger, target: &mut Target, path: P) -> Self {
        let (texture, sampler) = load_texture(log, target, path,
            || format::R8G8B8A8Unorm,
            |target, buffer, texture| target.queue_unorm_texture_copy(buffer, texture),
        );

        TextureLinear {
            texture,
            sampler,
        }
    }

    pub fn uniform(&self) -> (Arc<ImmutableImage<format::R8G8B8A8Unorm>>, Arc<Sampler>) {
        (self.texture.clone(), self.sampler.clone())
    }
}

fn load_texture<
    F: FormatDesc,
    FF: Fn() -> F,
    QF: Fn(&mut Target, Arc<CpuAccessibleBuffer<[[u8; 4]]>>, Arc<ImmutableImage<F>>),
    P: AsRef<Path>
>(
    log: &Logger, target: &mut Target, path: P, format_factory: FF, queue_func: QF,
) -> (Arc<ImmutableImage<F>>, Arc<Sampler>) {
    // Load in the image file
    info!(log, "Loading texture"; "path" => path.as_ref().display().to_string());
    let img = image::open(path.as_ref()).unwrap();
    let img_dimensions = img.dimensions();

    // Load the image data into a buffer
    let buffer = {
        let image_data = img.raw_pixels();
        let image_data_chunks = image_data.chunks(4).map(|c| [c[0], c[1], c[2], c[3]]);

        // TODO: staging buffer instead
        CpuAccessibleBuffer::<[[u8; 4]]>::from_iter(
            target.device().clone(), BufferUsage::all(),
            Some(target.graphics_queue().family()), image_data_chunks
        ).unwrap()
    };

    // Create the texture and sampler for the image, the texture data will later be copied in
    //  a command buffer
    let texture = ImmutableImage::new(
        target.device().clone(),
        Dimensions::Dim2d { width: img_dimensions.0, height: img_dimensions.1 },
        format_factory(), Some(target.graphics_queue().family())
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
    queue_func(target, buffer, texture.clone());

    (texture, sampler)
}
