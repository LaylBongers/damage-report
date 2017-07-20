use window::{Window, AdvancedWindow};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer, Types};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DTypes};
use calcium_rendering_world3d::{World3DTypes, World3DRenderTarget};
use calcium_rendering_static::{Initializer};

use view::{UiView, ViewportView};
use model::{MapEditor};
use input_manager::{InputManager};

pub struct WindowController<W: Window, T: Types, WT: World3DTypes<T>, ST: Simple2DTypes<T>> {
    window: W,
    window_renderer: T::WindowRenderer,

    simple2d_rendertarget: Simple2DRenderTarget<T, ST>,
    world3d_rendertarget: World3DRenderTarget<T, WT>,
}

impl<W: Window + AdvancedWindow, T: Types, WT: World3DTypes<T>, ST: Simple2DTypes<T>>
    WindowController<W, T, WT, ST> {
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

        // Set up 3D viewport rendering
        let world3d_rendertarget = World3DRenderTarget::new(
            true, renderer, &window_renderer, world3d_renderer
        );

        Ok(WindowController {
            window,
            window_renderer,

            simple2d_rendertarget,
            world3d_rendertarget,
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

        let mut app = MapEditor::new();
        let mut viewport = ViewportView::new(renderer, &mut app)?;
        let mut ui = UiView::new(self.window_renderer.size(), renderer)?;

        while !self.window.should_close() {
            let delta = timer.tick();

            // Poll for window events
            input.new_frame();
            while let Some(event) = self.window.poll_event() {
                // Let the initializer handle anything needed
                init.handle_event(&event, renderer, &mut self.window, &mut self.window_renderer);

                // Update the input manager with this event
                input.handle_event(&event);

                // Pass the event to the ui
                ui.handle_event(&event, &self.window_renderer);
            }

            // Update the UI
            ui.update(delta, &mut app);
            input.cursor_over_ui = ui.cursor_over_ui();

            // Update the viewport
            viewport.update(delta, &input, &mut self.window);

            // Render everything
            let mut frame = self.window_renderer.start_frame(renderer);
            viewport.render(
                &mut frame,
                renderer, &mut self.window_renderer,
                world3d_renderer, &mut self.world3d_rendertarget,
            );
            ui.render(
                &mut frame,
                renderer, &mut self.window_renderer,
                simple2d_renderer, &mut self.simple2d_rendertarget,
            )?;
            self.window_renderer.finish_frame(renderer, frame);
            self.window.swap_buffers();
        }

        Ok(())
    }
}
