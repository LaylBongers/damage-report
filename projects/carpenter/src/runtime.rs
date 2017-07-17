use cgmath::{Vector3, Quaternion, One};

use window::{Window, WindowSettings};
use slog::{Logger};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer};
use calcium_rendering_static::{Runtime, Initializer};
use calcium_conrod::{ConrodRenderer};

use editor_ui::{EditorUi};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Set up the renderer and open up a window to render to
        let window_settings = WindowSettings::new("Carpenter", [1280, 720]);
        let (mut renderer, mut window, mut window_renderer) =
            init.renderer(Some(self.log.clone()), &window_settings)?;

        // Set up 2D rendering for the UI
        let mut simple2d_renderer = init.simple2d_renderer(&mut renderer)?;
        let mut simple2d_render_target = Simple2DRenderTarget::new(
            false, &renderer, &window_renderer, &simple2d_renderer
        );

        // Set up 3D viewport rendering
        let mut world3d_renderer = init.world3d_renderer(&mut renderer, &mut window_renderer)?;
        let camera = Camera::new(
            Vector3::new(0.0, 0.0, -5.0),
            Quaternion::one(),
        );
        let mut render_world = RenderWorld::new();
        render_world.ambient_light = Vector3::new(0.005, 0.005, 0.005);

        // Set up conrod and UI data
        let mut conrod_renderer = ConrodRenderer::new(&mut renderer)?;
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
            world3d_renderer.render(
                &render_world, &camera,
                &mut renderer, &mut window_renderer, &mut frame
            );
            simple2d_renderer.render(
                &ui_batches, &mut simple2d_render_target,
                &mut renderer, &mut window_renderer, &mut frame
            );
            window_renderer.finish_frame(&mut renderer, frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
