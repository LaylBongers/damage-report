use cgmath::{Vector3, Quaternion, One};

use window::{Window};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer, Types, Texture, TextureFormat};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer, Simple2DTypes, RenderBatch};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer, Entity, World3DTypes, Model, Material};
use calcium_rendering_static::{Initializer};
use calcium_conrod::{ConrodRenderer};

use editor::ui::{EditorUi};

pub struct EditorWindow<W: Window, T: Types, WT: World3DTypes<T>, ST: Simple2DTypes<T>> {
    window: W,
    window_renderer: T::WindowRenderer,

    simple2d_rendertarget: Simple2DRenderTarget<T, ST>,
    conrod_renderer: ConrodRenderer<T>,
    ui: EditorUi,
    ui_batches: Vec<RenderBatch<T>>,

    world3d_renderer: WT::Renderer,
    render_world: RenderWorld<T, WT>,
    camera: Camera,
}

impl<W: Window, T: Types, WT: World3DTypes<T>, ST: Simple2DTypes<T>> EditorWindow<W, T, WT, ST> {
    pub fn new<I: Initializer<Window = W, Types=T, World3DTypes=WT, Simple2DTypes=ST>>(
        init: &I,
        renderer: &mut T::Renderer,
        simple2d_renderer: &ST::Renderer,
        window: W, mut window_renderer: T::WindowRenderer,
    ) -> Result<Self, Error> {
        // Set up 2D UI rendering
        let simple2d_rendertarget = Simple2DRenderTarget::new(
            false, renderer, &window_renderer, simple2d_renderer
        );
        let conrod_renderer = ConrodRenderer::new(renderer)?;
        let ui_batches = vec!();
        let ui = EditorUi::new(window_renderer.size());

        // Set up 3D viewport rendering
        let world3d_renderer = init.world3d_renderer(renderer, &mut window_renderer)?;
        let camera = Camera::new(
            Vector3::new(0.0, 2.0, 5.0),
            Quaternion::one(),
        );
        let mut render_world = RenderWorld::new();
        seed_world(&mut render_world, renderer)?;

        Ok(EditorWindow {
            window,
            window_renderer,

            simple2d_rendertarget,
            conrod_renderer,
            ui,
            ui_batches,

            world3d_renderer,
            render_world,
            camera,
        })
    }

    pub fn run_loop<I: Initializer<Window = W, Types=T, World3DTypes=WT, Simple2DTypes=ST>>(
        &mut self,
        init: &I,
        renderer: &mut <I::Types as Types>::Renderer,
        simple2d_renderer: &mut <I::Simple2DTypes as Simple2DTypes<I::Types>>::Renderer,
    ) -> Result<(), Error> {
        let mut timer = LoopTimer::start();

        while !self.window.should_close() {
            let delta = timer.tick();

            // Poll for window events
            while let Some(event) = self.window.poll_event() {
                // Let the initializer handle anything needed
                init.handle_event(&event, renderer, &mut self.window, &mut self.window_renderer);

                // Pass the event to conrod
                let size = self.window_renderer.size();
                if let Some(event) = ::conrod::backend::piston::event::convert(
                    event.clone(), size.x as f64, size.y as f64
                ) {
                    self.ui.ui.handle_event(event);
                }
            }

            // Update the UI
            self.ui.update(delta);

            // Create render batches for the UI
            if let Some(changed_batches) = self.conrod_renderer.draw_if_changed(
                renderer, &self.window_renderer, &mut self.ui.ui
            )? {
                self.ui_batches = changed_batches;
            }

            // Perform the rendering itself
            let mut frame = self.window_renderer.start_frame(renderer);
            self.world3d_renderer.render(
                &self.render_world, &self.camera,
                renderer, &mut self.window_renderer, &mut frame
            );
            simple2d_renderer.render(
                &self.ui_batches, &mut self.simple2d_rendertarget,
                renderer, &mut self.window_renderer, &mut frame
            );
            self.window_renderer.finish_frame(renderer, frame);
            self.window.swap_buffers();
        }

        Ok(())
    }
}

fn seed_world<T: Types, WT: World3DTypes<T>>(
    world: &mut RenderWorld<T, WT>, renderer: &mut T::Renderer
) -> Result<(), Error> {
    world.ambient_light = Vector3::new(0.05, 0.05, 0.05);
    world.directional_light = Vector3::new(1.0, 1.0, 1.0);

    let model = Model::<T, WT>::load(renderer, "./assets/cube.obj", 1.0);
    let material = Material {
        base_color: Texture::from_file(
            renderer, "./assets/texture.png", TextureFormat::Srgb
        )?,
        normal_map: Texture::from_file(
            renderer, "./assets/texture_normal.png", TextureFormat::Linear
        )?,
        metallic_map: Texture::from_file(
            renderer, "./assets/texture_metallic.png", TextureFormat::LinearRed
        )?,
        roughness_map: Texture::from_file(
            renderer, "./assets/texture_roughness.png", TextureFormat::LinearRed
        )?,
    };

    world.add_entity(Entity {
        position: Vector3::new(0.0, 0.0, 0.0),
        mesh: model.meshes[0].clone(),
        material: material,
    });

    Ok(())
}
