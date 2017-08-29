use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2};
use image::{self, GenericImage};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::image::{Dimensions};
use vulkano::image::immutable::{ImmutableImage};

use calcium_rendering::{CalciumErrorMappable, Error, Renderer};
use calcium_rendering::texture::{TextureSource, TextureRaw, TextureBuilder, TextureStoreFormat};
use {VulkanoRenderer};

pub struct VulkanoTextureRaw {
    image: Arc<ImmutableImage<Format>>,
}

impl VulkanoTextureRaw {
    fn from_buffer(
        buffer: Arc<CpuAccessibleBuffer<[u8]>>,
        size: Vector2<u32>,
        builder: TextureBuilder<VulkanoRenderer>,
        renderer: &mut VulkanoRenderer,
    ) -> Result<Self, Error> {
        // Get the correct format for the srgb parameter we got passed
        let format = match builder.store_format {
            TextureStoreFormat::Srgb => Format::R8G8B8A8Srgb,
            TextureStoreFormat::Linear => Format::R8G8B8A8Unorm,
            TextureStoreFormat::SingleChannel => Format::R8Unorm,
        };

        // Create the texture and sampler for the image, the texture data will later be copied in
        //  a command buffer
        let (image, command_buffer_exec) = ImmutableImage::from_buffer(
            buffer,
            Dimensions::Dim2d { width: size.x, height: size.y },
            format,
            renderer.graphics_queue().clone(),
        ).map_platform_err()?;

        // Queue copying the data to the image so it will be available when rendering
        renderer.queue_image_copy(command_buffer_exec);

        Ok(VulkanoTextureRaw {
            image,
        })
    }

    pub fn image(&self) -> &Arc<ImmutableImage<Format>> {
        &self.image
    }
}

impl TextureRaw<VulkanoRenderer> for VulkanoTextureRaw {
    fn new(
        builder: TextureBuilder<VulkanoRenderer>, renderer: &mut VulkanoRenderer
    ) -> Result<Self, Error> {
        let (buffer, size) = match builder.source {
            TextureSource::File(ref path) =>
                buffer_and_size_from_path(path, &builder, renderer)?,
            TextureSource::GreyscaleBytes { bytes, size } => {
                // TODO: Make this just a warning and support conversion
                if builder.store_format != TextureStoreFormat::SingleChannel {
                    panic!("Vulkano backend does not support converting greyscale to multi-channel")
                }
                (buffer_from_greyscale_bytes(bytes, size, renderer)?, size)
            },
        };

        Self::from_buffer(
            buffer, size, builder, renderer
        )
    }
}

fn buffer_and_size_from_path(
    path: &PathBuf, builder: &TextureBuilder<VulkanoRenderer>, renderer: &mut VulkanoRenderer
) -> Result<(Arc<CpuAccessibleBuffer<[u8]>>, Vector2<u32>), Error> {
    info!(renderer.log(),
        "Loading texture from file"; "path" => path.display().to_string()
    );

    // Load in the image file
    let img = image::open(path).unwrap();
    let img_dimensions = img.dimensions();

    // Load the image data into a buffer
    let buffer = {
        let image_data = img.to_rgba().into_raw();

        // If the format is LinearRed, we need to ignore the GBA elements
        let chunk_size = if builder.store_format != TextureStoreFormat::SingleChannel { 1 } else { 4 };
        let image_data_iter = image_data.chunks(chunk_size).map(|c| c[0]);

        // TODO: Use staging buffer instead
        CpuAccessibleBuffer::<[u8]>::from_iter(
            renderer.device().clone(), BufferUsage::all(), image_data_iter
        ).unwrap()
    };

    Ok((buffer, Vector2::new(img_dimensions.0, img_dimensions.1)))
}

fn buffer_from_greyscale_bytes(
    bytes: &[u8], size: Vector2<u32>, renderer: &mut VulkanoRenderer
) -> Result<Arc<CpuAccessibleBuffer<[u8]>>, Error> {
    info!(renderer.log(),
        "Loading texture from greyscale data"; "width" => size.x, "height" => size.y
    );

    // Load the image data into a buffer
    let buffer = {
        let image_data_iter = bytes.iter().map(|v| *v);

        // TODO: Use staging buffer instead
        CpuAccessibleBuffer::<[u8]>::from_iter(
            renderer.device().clone(), BufferUsage::all(), image_data_iter
        ).unwrap()
    };

    Ok(buffer)
}
