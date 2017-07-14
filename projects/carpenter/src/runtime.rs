use cgmath::{Vector2};
use window::{Window, WindowSettings};
use slog::{Logger};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderer};
use calcium_rendering_static::{Runtime, Initializer};
use calcium_conrod::{ConrodRenderer};

use editor_ui::{EditorUi};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        let size = Vector2::new(1280, 720);

        // Set up everything we need to render
        let size_a: [u32; 2] = size.into();
        let window_settings = WindowSettings::new("Carpenter", size_a);
        let (mut renderer, mut window, mut window_renderer) =
            init.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = init.simple2d_renderer(&mut renderer)?;

        // Set up conrod and UI data
        let mut conrod_renderer: ConrodRenderer<I::Types> =
            ConrodRenderer::new(&mut renderer)?;
        let mut ui_batches = vec!();
        let mut editor_ui = EditorUi::new(size);

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        info!(self.log, "Finished loading, starting main loop");
        while !window.should_close() {
            let delta = timer.tick();

            // Poll for window events
            while let Some(event) = window.poll_event() {
                // Pass the event to the window renderer
                window_renderer.handle_event(&event);

                // Pass the event to conrod
                let size = window_renderer.size();
                if let Some(event) = ::conrod::backend::piston::event::convert(
                    event.clone(), size.x as f64, size.y as f64
                ) {
                    editor_ui.ui.handle_event(event);
                }
            }

            // Update the UI
            editor_ui.update(delta);

            // Create render batches for the UI
            // TODO: ui_batches to an Option return value
            conrod_renderer.draw_if_changed(
                &mut renderer, &window_renderer, &mut editor_ui.ui, &mut ui_batches
            )?;

            // Perform the rendering itself
            let mut frame = window_renderer.start_frame(&renderer);
            simple2d_renderer.render(&mut renderer, &mut frame, &ui_batches);
            window_renderer.finish_frame(&renderer, frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
