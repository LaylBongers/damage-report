use window::{Window, AdvancedWindow};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer, Types};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer, Simple2DTypes, RenderBatch};
use calcium_rendering_world3d::{World3DTypes, World3DRenderTarget};
use calcium_rendering_static::{Initializer};
use calcium_conrod::{ConrodRenderer};

use editor::ui::{EditorUi};
use editor::viewport::{EditorViewport};
use input_manager::{InputManager};

pub struct EditorWindow<W: Window, T: Types, WT: World3DTypes<T>, ST: Simple2DTypes<T>> {
    window: W,
    window_renderer: T::WindowRenderer,

    simple2d_rendertarget: Simple2DRenderTarget<T, ST>,
    conrod_renderer: ConrodRenderer<T>,
    ui: EditorUi,
    ui_batches: Vec<RenderBatch<T>>,

    world3d_rendertarget: World3DRenderTarget<T, WT>,
    viewport: EditorViewport<T, WT>,
}

impl<W: Window + AdvancedWindow, T: Types, WT: World3DTypes<T>, ST: Simple2DTypes<T>>
    EditorWindow<W, T, WT, ST> {
    pub fn new(
        renderer: &mut T::Renderer,
        simple2d_renderer: &ST::Renderer,
        world3d_renderer: &WT::Renderer,
        window: W, window_renderer: T::WindowRenderer,
    ) -> Result<Self, Error> {
        // Set up 2D UI rendering
        let simple2d_rendertarget = Simple2DRenderTarget::new(
            false, renderer, &window_renderer, simple2d_renderer
        );
        let conrod_renderer = ConrodRenderer::new(renderer)?;
        let ui_batches = vec!();
        let ui = EditorUi::new(window_renderer.size());

        // Set up 3D viewport rendering
        let world3d_rendertarget = World3DRenderTarget::new(
            true, renderer, &window_renderer, world3d_renderer
        );

        let viewport = EditorViewport::new(renderer)?;

        Ok(EditorWindow {
            window,
            window_renderer,

            simple2d_rendertarget,
            conrod_renderer,
            ui,
            ui_batches,

            world3d_rendertarget,
            viewport,
        })
    }

    pub fn run_loop<I: Initializer<Window=W, Types=T, World3DTypes=WT, Simple2DTypes=ST>>(
        &mut self,
        init: &I,
        renderer: &mut T::Renderer,
        simple2d_renderer: &mut ST::Renderer,
        world3d_renderer: &mut WT::Renderer,
    ) -> Result<(), Error> {
        let mut input = InputManager::new();
        let mut timer = LoopTimer::start();

        while !self.window.should_close() {
            let delta = timer.tick();

            // Poll for window events
            while let Some(event) = self.window.poll_event() {
                // Let the initializer handle anything needed
                init.handle_event(&event, renderer, &mut self.window, &mut self.window_renderer);

                // Update the input manager with this event
                input.handle_event(&event);

                // Pass the event to conrod
                let size = self.window_renderer.size();
                if let Some(event) = ::conrod::backend::piston::event::convert(
                    event.clone(), size.x as f64, size.y as f64
                ) {
                    self.ui.ui.handle_event(event);
                }
            }

            // Update the UI and viewport
            self.ui.update(delta);
            self.viewport.update(delta, &input, &mut self.window);

            // Create render batches for the UI
            if let Some(changed_batches) = self.conrod_renderer.draw_if_changed(
                renderer, &self.window_renderer, &mut self.ui.ui
            )? {
                self.ui_batches = changed_batches;
            }

            // Perform the rendering itself
            let mut frame = self.window_renderer.start_frame(renderer);
            self.viewport.render(
                &mut frame,
                renderer, &mut self.window_renderer,
                world3d_renderer, &mut self.world3d_rendertarget,
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
