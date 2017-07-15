use std::sync::{Arc};

use cgmath::{self, Vector2};
use gfx::{self, Device, Factory, VertexBuffer, ConstantBuffer, TextureSampler};
use gfx::handle::{Sampler, Buffer};
use gfx::pso::{PipelineState};
use gfx::traits::{FactoryExt};
use gfx::texture::{SamplerInfo, FilterMethod, WrapMode};

use calcium_rendering::{Error, Texture};
use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxFrame, ColorFormat};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch, ShaderMode, SampleMode};

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "v_position",
        uv: [f32; 2] = "v_uv",
        color: [f32; 4] = "v_color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_transform",
    }

    constant Mode {
        mode: u32 = "u_mode",
    }

    pipeline pipe {
        vbuf: VertexBuffer<Vertex> = (),
        transform: ConstantBuffer<Transform> = "Transform",
        mode: ConstantBuffer<Mode> = "Mode",
        texture: TextureSampler<[f32; 4]> = "u_texture",
        out: gfx::BlendTarget<ColorFormat> = (
            "Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA
        ),
    }
}

pub struct GfxSimple2DRenderer<D: Device + 'static, F: Factory<D::Resources> + 'static> {
    pso: PipelineState<D::Resources, pipe::Meta>,
    dummy_texture: Arc<Texture<GfxTypes<D, F>>>,
    mode_buffers: Vec<Buffer<D::Resources, Mode>>,

    linear_sampler: Sampler<D::Resources>,
    nearest_sampler: Sampler<D::Resources>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static> GfxSimple2DRenderer<D, F> {
    pub fn new(
        renderer: &mut GfxRenderer<D, F>
    ) -> Result<Self, Error> {
        let pso = renderer.factory.create_pipeline_simple(
            include_bytes!("../shaders/simple2d_150_vert.glsl"),
            include_bytes!("../shaders/simple2d_150_frag.glsl"),
            pipe::new()
        ).unwrap();

        let dummy_texture = Texture::from_raw_greyscale(
            renderer, &vec![255u8; 8*8], Vector2::new(8, 8)
        )?;

        // Create pre-made buffers for the shader modes
        let mut mode_buffers = Vec::new();
        for i in 0..3 {
            let mode = Mode {
                mode: i
            };
            let mode_buffer = renderer.factory.create_constant_buffer(1);
            renderer.encoder.update_buffer(&mode_buffer, &[mode], 0).unwrap();
            mode_buffers.push(mode_buffer);
        }

        // Create the samplers for the two sample modes
        let linear_sampler = renderer.factory.create_sampler(SamplerInfo::new(
            FilterMethod::Trilinear,
            WrapMode::Clamp,
        ));
        let nearest_sampler = renderer.factory.create_sampler(SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Clamp,
        ));

        Ok(GfxSimple2DRenderer {
            pso,
            dummy_texture,
            mode_buffers,

            linear_sampler,
            nearest_sampler,
        })
    }

    fn sampler_for_mode(&self, sample_mode: &SampleMode) -> &Sampler<D::Resources> {
        match sample_mode {
            &SampleMode::Linear => &self.linear_sampler,
            &SampleMode::Nearest => &self.nearest_sampler,
        }
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderer<GfxTypes<D, F>> for GfxSimple2DRenderer<D, F> {
    fn render(
        &mut self, renderer: &mut GfxRenderer<D, F>, frame: &mut GfxFrame,
        batches: &[RenderBatch<GfxTypes<D, F>>]
    ) {
        // Create a projection matrix that just matches coordinates to pixels
        let proj = cgmath::ortho(
            0.0, frame.size.x as f32,
            frame.size.y as f32, 0.0,
            1.0, -1.0
        );
        let transform = Transform {
            transform: proj.into()
        };
        let transform_buffer = renderer.factory.create_constant_buffer(1);
        renderer.encoder.update_buffer(&transform_buffer, &[transform], 0).unwrap();

        // Go over all batches
        for batch in batches {
            // Create a big mesh of all the rectangles we got told to draw this batch
            let mut vertices = Vec::new();
            for vertex in &batch.vertices {
                vertices.push(Vertex {
                    position: vertex.position.into(),
                    uv: vertex.uv.into(),
                    color: vertex.color.into(),
                });
            }

            // Create an actual VBO from it
            let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(
                &vertices, ()
            );

            // Get the mode ID this batch has and a texture to render
            // TODO: Figure out a way to avoid having to have a dummy texture
            let (mode_id, texture, sampler) = match &batch.mode {
                &ShaderMode::Color =>
                    (0, &self.dummy_texture, &self.linear_sampler),
                &ShaderMode::Texture(ref texture, ref sample_mode) =>
                    (1, texture, self.sampler_for_mode(sample_mode)),
                &ShaderMode::Mask(ref texture, ref sample_mode) =>
                    (2, texture, self.sampler_for_mode(sample_mode)),
            };

            // Get the matching buffer for this shader mode
            let mode_buffer = &self.mode_buffers[mode_id];

            // Gather together all the data we need to render
            let data = pipe::Data {
                vbuf: vertex_buffer,
                transform: transform_buffer.clone(),
                mode: mode_buffer.clone(),
                texture: (texture.raw.view.clone(), sampler.clone()),
                out: renderer.color_view.clone(),
            };

            // Finally, add the draw to the encoder
            renderer.encoder.draw(&slice, &self.pso, &data);
        }
    }
}
