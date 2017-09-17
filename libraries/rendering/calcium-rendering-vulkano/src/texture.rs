use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2};
use image::{self};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::image::{Dimensions, MipmapsCount, ImageUsage, ImageLayout};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sampler::{Sampler, Filter, MipmapMode, SamplerAddressMode};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};

use calcium_rendering::{CalciumErrorMappable, Error, Renderer};
use calcium_rendering::texture::{TextureSource, TextureRaw, TextureBuilder, TextureStoreFormat, SampleMode};
use {VulkanoRenderer};

pub struct VulkanoTextureRaw {
    image: Arc<ImmutableImage<Format>>,
    sampler: Arc<Sampler>,
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

        // TODO: I have no idea how these two values interact, it seems 0 is default in some places
        // for no mipmaps and One in some others.
        let (mipmap_levels, mipmaps_count) = if builder.generate_mipmaps {
            let levels = mip_levels(size.x, size.y);
            (levels, MipmapsCount::Specific(levels))
        } else {
            (0, MipmapsCount::One)
        };

        // Create the image itself with an initializer
        let dimensions = Dimensions::Dim2d { width: size.x, height: size.y };
        let (image, initializer) = ImmutableImage::uninitialized(
            renderer.device().clone(),
            dimensions,
            format,
            mipmaps_count,
            ImageUsage {
                transfer_source: true, transfer_destination: true, sampled: true,
                ..ImageUsage::none()
            },
            // TODO: Switch between layouts while generating mipmaps then switch to ShaderReadOnlyOptimal
            ImageLayout::ShaderReadOnlyOptimal,
            renderer.device().active_queue_families(),
        ).map_platform_err()?;
        let initializer = Arc::new(initializer);

        // Copy over the base image
        let mut cbb = AutoCommandBufferBuilder::new(
                renderer.device().clone(), renderer.graphics_queue().family()
            ).map_platform_err()?
            .copy_buffer_to_image_dimensions(
                buffer, initializer.clone(),
                [0, 0, 0], dimensions.width_height_depth(),
                0, dimensions.array_layers_with_cube(),
                // This last one is the target mipmap level
                0
            ).unwrap();

        // Now generate all the mipmap levels
        for level in 1..mipmap_levels {
            cbb = cbb.blit_image(
                // Source
                initializer.clone(),
                [0, 0, 0],
                [
                    (size.x >> (level - 1)) as i32,
                    (size.y >> (level - 1)) as i32,
                    1,
                ],
                0,
                level - 1,

                // Destination
                initializer.clone(),
                [0, 0, 0],
                [
                    (size.x >> level) as i32,
                    (size.y >> level) as i32,
                    1,
                ],
                0,
                level,

                1,
                Filter::Linear,
            ).unwrap();
        }

        // Submit all those copy and blit commands to be done before the next frame
        let future = match cbb.build().unwrap().execute(renderer.graphics_queue().clone()) {
            Ok(f) => f,
            Err(_) => unreachable!(),
        };
        renderer.queue_command_buffer_future(future);

        // Create a sampler for this texture based on our mipmapping data (if any)
        let filter = if builder.sample_mode == SampleMode::Linear {
            Filter::Linear
        } else {
            Filter::Nearest
        };
        let sampler = Sampler::new(
            renderer.device().clone(),
            filter,
            filter,
            MipmapMode::Linear,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0,
            // TODO: Only enable anisotropic filtering if it's supported
            // TODO: This is a performance hit, let the user define how strong it should be
            16.0,
            0.0, mipmap_levels as f32
        ).map_platform_err()?;

        Ok(VulkanoTextureRaw {
            image,
            sampler,
        })
    }

    pub fn image(&self) -> &Arc<ImmutableImage<Format>> {
        &self.image
    }

    pub fn sampler(&self) -> &Arc<Sampler> {
        &self.sampler
    }
}

impl TextureRaw<VulkanoRenderer> for VulkanoTextureRaw {
    fn new(
        builder: TextureBuilder<VulkanoRenderer>, renderer: &mut VulkanoRenderer
    ) -> Result<Self, Error> {
        let (buffer, size) = match builder.source {
            TextureSource::File(ref path) =>
                buffer_and_size_from_path(path, &builder, renderer)?,
            TextureSource::Bytes { ref bytes, size, color } => {
                // TODO: Make these just a warnings and support conversion
                if color && builder.store_format == TextureStoreFormat::SingleChannel {
                    panic!("Vulkano backend does not support converting color bytes to single-channel, this needs to be added!")
                }
                if !color && builder.store_format != TextureStoreFormat::SingleChannel {
                    panic!("Vulkano backend does not support converting greyscale bytes to color, this needs to be added!")
                }
                (buffer_from_bytes(bytes.as_ref(), size, color, renderer)?, size)
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
    let img = image::open(path).unwrap().to_rgba();
    let img_dimensions = img.dimensions();
    let size = Vector2::new(img_dimensions.0, img_dimensions.1);

    // Load the image data into a buffer
    let buffer = {
        let image_data = img.into_raw();

        // If the format is LinearRed, we need to ignore the GBA elements
        let chunk_size = if builder.store_format != TextureStoreFormat::SingleChannel { 1 } else { 4 };
        let image_data_iter = image_data.chunks(chunk_size).map(|c| c[0]);

        // TODO: Use staging buffer instead
        CpuAccessibleBuffer::<[u8]>::from_iter(
            renderer.device().clone(), BufferUsage::all(), image_data_iter
        ).unwrap()
    };

    Ok((buffer, size))
}

fn buffer_from_bytes(
    bytes: &[u8], size: Vector2<u32>, color: bool, renderer: &mut VulkanoRenderer
) -> Result<Arc<CpuAccessibleBuffer<[u8]>>, Error> {
    info!(renderer.log(),
        "Loading texture from bytes"; "width" => size.x, "height" => size.y, "color" => color
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

pub fn mip_levels(width: u32, height: u32) -> u32 {
    (width as f32).max(height as f32).log2().floor() as u32 + 1
}
