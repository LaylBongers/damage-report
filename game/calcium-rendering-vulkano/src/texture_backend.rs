use std::sync::{Arc};
use std::path::{PathBuf};

use slog::{Logger};
use image::{self, GenericImage};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::image::{Dimensions};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};

use calcium_rendering::texture::{TextureFormat, TextureBackend};
use {VulkanoBackendTypes, VulkanoRenderBackend};

pub struct VulkanoTextureBackend {
    image: Arc<ImmutableImage<Format>>,
    sampler: Arc<Sampler>,
}

impl VulkanoTextureBackend {
    pub fn uniform(&self) -> (Arc<ImmutableImage<Format>>, Arc<Sampler>) {
        (self.image.clone(), self.sampler.clone())
    }
}

impl TextureBackend<VulkanoBackendTypes> for VulkanoTextureBackend {
    fn new(
        log: &Logger, backend: &mut VulkanoRenderBackend, path: PathBuf, format: TextureFormat
    ) -> Self {
        // Load in the image file
        info!(log, "Loading texture"; "path" => path.display().to_string());
        let img = image::open(path).unwrap();
        let img_dimensions = img.dimensions();

        // Load the image data into a buffer
        let buffer = {
            let image_data = img.to_rgba().into_raw();

            // If the format is LinearRed, we need to ignore the GBA elements
            let chunk_size = if format != TextureFormat::LinearRed { 1 } else { 4 };
            let image_data_iter = image_data.chunks(chunk_size).map(|c| c[0]);

            // TODO: staging buffer instead
            CpuAccessibleBuffer::<[u8]>::from_iter(
                backend.device.clone(), BufferUsage::all(),
                Some(backend.graphics_queue.family()), image_data_iter
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
        let image = ImmutableImage::new(
            backend.device.clone(),
            Dimensions::Dim2d { width: img_dimensions.0, height: img_dimensions.1 },
            format, Some(backend.graphics_queue.family())
        ).unwrap();
        let sampler = Sampler::new(
            backend.device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).unwrap();

        // Queue copying the data to the image so it will be available when rendering
        backend.queue_image_copy(buffer, image.clone());

        VulkanoTextureBackend {
            image,
            sampler,
        }
    }
}
