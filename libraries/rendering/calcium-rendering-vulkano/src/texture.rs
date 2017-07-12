use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2};
use image::{self, GenericImage};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::image::{Dimensions};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};

use calcium_rendering::{TextureFormat, Texture, CalciumErrorMappable, Error};
use {VulkanoBackendTypes, VulkanoRenderer};

pub struct VulkanoTexture {
    image: Arc<ImmutableImage<Format>>,
    sampler: Arc<Sampler>,
}

impl VulkanoTexture {
    fn from_buffer(
        renderer: &mut VulkanoRenderer,
        buffer: Arc<CpuAccessibleBuffer<[u8]>>, size: Vector2<u32>,
        format: TextureFormat
    ) -> Result<Arc<Self>, Error> {
        // Get the correct format for the srgb parameter we got passed
        let format = match format {
            TextureFormat::Srgb => Format::R8G8B8A8Srgb,
            TextureFormat::Linear => Format::R8G8B8A8Unorm,
            TextureFormat::LinearRed => Format::R8Unorm,
        };

        // Create the texture and sampler for the image, the texture data will later be copied in
        //  a command buffer
        let image = ImmutableImage::new(
            renderer.device.clone(),
            Dimensions::Dim2d { width: size.x, height: size.y },
            format, Some(renderer.graphics_queue.family())
        ).map_platform_err()?;
        let sampler = Sampler::new(
            renderer.device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).map_platform_err()?;

        // Queue copying the data to the image so it will be available when rendering
        renderer.queue_image_copy(buffer, image.clone());

        Ok(Arc::new(VulkanoTexture {
            image,
            sampler,
        }))
    }

    pub fn uniform(&self) -> (Arc<ImmutableImage<Format>>, Arc<Sampler>) {
        (self.image.clone(), self.sampler.clone())
    }
}

impl Texture<VulkanoBackendTypes> for VulkanoTexture {
    fn from_file(
        renderer: &mut VulkanoRenderer, path: PathBuf, format: TextureFormat
    ) -> Result<Arc<Self>, Error> {
        info!(renderer.log,
            "Loading texture from file"; "path" => path.display().to_string()
        );

        // Load in the image file
        let img = image::open(path).unwrap();
        let img_dimensions = img.dimensions();

        // Load the image data into a buffer
        let buffer = {
            let image_data = img.to_rgba().into_raw();

            // If the format is LinearRed, we need to ignore the GBA elements
            let chunk_size = if format != TextureFormat::LinearRed { 1 } else { 4 };
            let image_data_iter = image_data.chunks(chunk_size).map(|c| c[0]);

            // TODO: Use staging buffer instead
            CpuAccessibleBuffer::<[u8]>::from_iter(
                renderer.device.clone(), BufferUsage::all(),
                Some(renderer.graphics_queue.family()), image_data_iter
            ).unwrap()
        };

        Self::from_buffer(
            renderer, buffer, Vector2::new(img_dimensions.0, img_dimensions.1), format
        )
    }

    fn from_raw_greyscale(
        renderer: &mut VulkanoRenderer, data: &[u8], size: Vector2<u32>,
    ) -> Result<Arc<Self>, Error> {
        info!(renderer.log,
            "Loading texture from greyscale data"; "width" => size.x, "height" => size.y
        );

        // Load the image data into a buffer
        let buffer = {
            let image_data_iter = data.iter().map(|v| *v);

            // TODO: Use staging buffer instead
            CpuAccessibleBuffer::<[u8]>::from_iter(
                renderer.device.clone(), BufferUsage::all(),
                Some(renderer.graphics_queue.family()), image_data_iter
            ).unwrap()
        };

        Self::from_buffer(
            renderer, buffer, size, TextureFormat::LinearRed
        )
    }
}
