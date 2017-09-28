use std::sync::{Arc};

use cgmath::{Vector2};
use gfx::{self, Device, Factory, VertexBuffer, ConstantBuffer};
use gfx::handle::{Sampler, Buffer};
use gfx::pso::{PipelineState};
use gfx::pso::resource::{RawShaderResource};
use gfx::traits::{FactoryExt};
use gfx::texture::{SamplerInfo, FilterMethod, WrapMode};

use calcium_rendering::raw::{RawAccess};
use calcium_rendering::texture::{Texture, SampleMode};
use calcium_rendering::{Error, Frame, Renderer};
use calcium_rendering_gfx::{GfxRendererRaw, ColorFormat};
use calcium_rendering_simple2d::render_data::{ShaderMode, RenderData, RenderSet};
use calcium_rendering_simple2d::raw::{Simple2DRendererRaw};
use calcium_rendering_simple2d::{Simple2DRenderTarget};

use {GfxSimple2DRenderTargetRaw};

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
        texture: RawShaderResource = "u_texture",
        texture_sampler: ::gfx::pso::resource::Sampler = "u_texture",
        out: gfx::BlendTarget<ColorFormat> = (
            "Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA
        ),
    }
}

pub struct GfxSimple2DRendererRaw<D: Device + 'static, F: Factory<D::Resources> + 'static> {
    pso: PipelineState<D::Resources, pipe::Meta>,
    dummy_texture: Arc<Texture<GfxRendererRaw<D, F>>>,
    mode_buffers: Vec<Buffer<D::Resources, Mode>>,

    linear_sampler: Sampler<D::Resources>,
    nearest_sampler: Sampler<D::Resources>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static> GfxSimple2DRendererRaw<D, F> {
    pub fn new(
        renderer: &mut Renderer<GfxRendererRaw<D, F>>
    ) -> Result<Self, Error> {
        let pso = renderer.raw_mut().factory_mut().create_pipeline_simple(
            include_bytes!("../shaders/simple2d_150_vert.glsl"),
            include_bytes!("../shaders/simple2d_150_frag.glsl"),
            pipe::new()
        ).unwrap();

        let dummy_texture = Texture::new()
            .from_bytes(vec![255u8; 8*8], Vector2::new(8, 8), false)
            .as_single_channel()
            .build(renderer)?;

        // Create pre-made buffers for the shader modes
        let mut mode_buffers = Vec::new();
        for i in 0..3 {
            let mode = Mode {
                mode: i
            };
            let mode_buffer = renderer.raw_mut().factory_mut().create_constant_buffer(1);
            renderer.raw_mut().encoder_mut().update_buffer(&mode_buffer, &[mode], 0).unwrap();
            mode_buffers.push(mode_buffer);
        }

        // Create the samplers for the two sample modes
        let linear_sampler = renderer.raw_mut().factory_mut().create_sampler(SamplerInfo::new(
            FilterMethod::Trilinear,
            WrapMode::Clamp,
        ));
        let nearest_sampler = renderer.raw_mut().factory_mut().create_sampler(SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Clamp,
        ));

        Ok(GfxSimple2DRendererRaw {
            pso,
            dummy_texture,
            mode_buffers,

            linear_sampler,
            nearest_sampler,
        })
    }

    fn sampler_for_mode(&self, sample_mode: SampleMode) -> &Sampler<D::Resources> {
        match sample_mode {
            SampleMode::Linear => &self.linear_sampler,
            SampleMode::Nearest => &self.nearest_sampler,
        }
    }

    fn render_set(
        &mut self,
        set: &RenderSet<GfxRendererRaw<D, F>>,
        frame: &mut Frame<GfxRendererRaw<D, F>>,
        renderer: &mut Renderer<GfxRendererRaw<D, F>>,
    ) {
        // Create a projection matrix that just matches coordinates to pixels
        let proj = set.projection.to_matrix(frame.raw().size());
        let transform = Transform {
            transform: proj.into()
        };
        let transform_buffer = renderer.raw_mut().factory_mut().create_constant_buffer(1);
        renderer.raw_mut().encoder_mut().update_buffer(&transform_buffer, &[transform], 0).unwrap();

        // Go over all batches
        for batch in &set.batches {
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
            let (vertex_buffer, slice) = renderer.raw_mut().factory_mut()
                .create_vertex_buffer_with_slice(
                    &vertices, ()
                );

            // Get the mode ID this batch has and a texture to render
            // TODO: Figure out a way to avoid having to have a dummy texture
            let (mode_id, texture, sampler) = match &batch.mode {
                &ShaderMode::Color =>
                    (0, &self.dummy_texture, &self.linear_sampler),
                &ShaderMode::Texture(ref texture) =>
                    (1, texture, self.sampler_for_mode(texture.raw().sample_mode)),
                &ShaderMode::Mask(ref texture) =>
                    (2, texture, self.sampler_for_mode(texture.raw().sample_mode)),
            };

            // Get the matching buffer for this shader mode
            let mode_buffer = &self.mode_buffers[mode_id];

            // Gather together all the data we need to render
            let data = pipe::Data {
                vbuf: vertex_buffer,
                transform: transform_buffer.clone(),
                mode: mode_buffer.clone(),
                texture: texture.raw().view.raw().clone(),
                texture_sampler: sampler.clone(),
                out: renderer.raw().color_view().clone(),
            };

            // Finally, add the draw to the encoder
            renderer.raw_mut().encoder_mut().draw(&slice, &self.pso, &data);
        }
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRendererRaw<GfxRendererRaw<D, F>> for GfxSimple2DRendererRaw<D, F>
{
    type RenderTargetRaw = GfxSimple2DRenderTargetRaw;

    fn render(
        &mut self,
        data: &RenderData<GfxRendererRaw<D, F>>,
        frame: &mut Frame<GfxRendererRaw<D, F>>,
        render_target: &mut Simple2DRenderTarget<GfxRendererRaw<D, F>, Self>,
        renderer: &mut Renderer<GfxRendererRaw<D, F>>,
    ) {
        // Clear if we were told to clear
        if render_target.raw.is_clear() {
            let color_view = renderer.raw().color_view().clone();
            renderer.raw_mut().encoder_mut().clear(&color_view, [0.0, 0.0, 0.0, 1.0]);
        }

        for set in &data.render_sets {
            self.render_set(set, frame, renderer);
        }
    }
}
