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

        // Set up everything we need to render
        let window_settings = WindowSettings::new("Carpenter", [1280, 720]);
        let (mut renderer, mut window, mut window_renderer) =
            init.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = init.simple2d_renderer(&mut renderer)?;

        // Set up conrod and UI data
        let mut conrod_renderer: ConrodRenderer<I::Types> =
            ConrodRenderer::new(&mut renderer)?;
        let mut ui_batches = vec!();
        let mut editor_ui = EditorUi::new(window_renderer.size());

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        info!(self.log, "Finished loading, starting main loop");
        while !window.should_close() {
            let delta = timer.tick();

            // Poll for window events
            while let Some(event) = window.poll_event() {
                // Let the initializer handle anything needed
                init.handle_event(&event, &mut renderer, &mut window, &mut window_renderer);

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
            if let Some(changed_batches) = conrod_renderer.draw_if_changed(
                &mut renderer, &window_renderer, &mut editor_ui.ui
            )? {
                ui_batches = changed_batches;
            }

            // Perform the rendering itself
            let mut frame = window_renderer.start_frame(&mut renderer);
            simple2d_renderer.render(&mut renderer, &mut frame, &ui_batches);
            window_renderer.finish_frame(&mut renderer, frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
