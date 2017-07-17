use cgmath::{Vector2};
use input::{Input, Button, Key};
use window::{Window, WindowSettings};
use slog::{Logger};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer, Texture, TextureFormat};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch, ShaderMode, DrawRectangle, Rectangle, SampleMode, Simple2DRenderTarget};
use calcium_rendering_static::{Runtime, Initializer};

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
        let mut simple2d_render_target = Simple2DRenderTarget::new(
            true, &renderer, &window_renderer, &simple2d_renderer
        );

        // Player data
        let player_texture = Texture::from_file(
            &mut renderer, "./assets/friendly.png", TextureFormat::Srgb,
        )?;
        let mut player: Vector2<f32> = Vector2::new(200.0, 200.0);
        let mut right_pressed = false;
        let player_size = Vector2::new(32.0, 32.0);

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        info!(self.log, "Finished loading, starting main loop");
        while !window.should_close() {
            let delta = timer.tick();

            // Handle input
            while let Some(event) = window.poll_event() {
                // Let the initializer handle anything needed
                init.handle_event(&event, &mut renderer, &mut window, &mut window_renderer);

                match event {
                    Input::Press(Button::Keyboard(Key::D)) =>
                        right_pressed = true,
                    Input::Release(Button::Keyboard(Key::D)) =>
                        right_pressed = false,
                    _ => {}
                }
            }

            // Move the player
            if right_pressed {
                // Move the player
                player.x += delta * 64.0;
            }

            // Render a textured square for the player
            let mut batches = Vec::new();
            let mut batch = RenderBatch::new(
                ShaderMode::Texture(player_texture.clone(), SampleMode::Nearest)
            );
            batch.rectangle(DrawRectangle::new(
                Rectangle::new(player - player_size/2.0, player + player_size/2.0)
            ));
            batches.push(batch);

            // Perform the rendering itself
            let mut frame = window_renderer.start_frame(&mut renderer);
            simple2d_renderer.render(
                &batches, &mut simple2d_render_target,
                &mut renderer, &mut window_renderer, &mut frame
            );
            window_renderer.finish_frame(&mut renderer, frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
