use calcium_rendering::{Renderer, Error, WindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderer, Simple2DRenderTarget};
use calcium_rendering_world3d::{World3DRenderer, World3DRenderTarget};
use calcium_rendering_static::{Initializer};
use window::{Window, AdvancedWindow, WindowSettings};
use slog::{Logger};

use carpenter_model::{MapEditor, InputModel};
use view::{ViewportView, UiView};

pub struct WindowView<W: Window, R: Renderer, SR: Simple2DRenderer<R>, WR: World3DRenderer<R>> {
    window: W,
    renderer: R,
    window_renderer: R::WindowRenderer,

    simple2d_renderer: SR,
    simple2d_rendertarget: Simple2DRenderTarget<R, SR>,

    world3d_renderer: WR,
    world3d_rendertarget: World3DRenderTarget<R, WR>,

    viewport: ViewportView<R, WR>,
    ui: UiView<R>,
}

impl<W: Window + AdvancedWindow, R: Renderer, SR: Simple2DRenderer<R>, WR: World3DRenderer<R>>
    WindowView<W, R, SR, WR> {
    pub fn new<I: Initializer<Window=W, Renderer=R, World3DRenderer=WR, Simple2DRenderer=SR>>(
        log: &Logger,
        init: &I,
        editor: &mut MapEditor,
    ) -> Result<Self, Error> {
        // Set up the renderers and open up a window to render to
        let window_settings = WindowSettings::new("Carpenter", [1280, 720]);
        let (mut renderer, window, window_renderer) =
            init.renderer(Some(log.clone()), &window_settings)?;

        let simple2d_renderer = init.simple2d_renderer(&mut renderer)?;
        let simple2d_rendertarget = Simple2DRenderTarget::new(
            false, &renderer, &window_renderer, &simple2d_renderer
        );

        let world3d_renderer = init.world3d_renderer(&mut renderer)?;
        let world3d_rendertarget = World3DRenderTarget::new(
            true, &renderer, &window_renderer, &world3d_renderer
        );

        let viewport = ViewportView::new(&mut renderer, editor)?;
        let ui = UiView::new(&mut renderer)?;

        Ok(WindowView {
            window,
            renderer,
            window_renderer,

            simple2d_renderer,
            simple2d_rendertarget,

            world3d_renderer,
            world3d_rendertarget,

            viewport,
            ui,
        })
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn handle_events<
        I: Initializer<Window=W, Renderer=R, World3DRenderer=WR, Simple2DRenderer=SR>
    >(
        &mut self, init: &I, input: &mut InputModel
    ) {
        input.new_frame();
        while let Some(event) = self.window.poll_event() {
            // Let the initializer handle anything needed
            init.handle_event(
                &event, &mut self.renderer, &mut self.window, &mut self.window_renderer
            );

            // Update the input manager with this event
            input.handle_event(&event);

            // Pass the event to the ui
            self.ui.handle_event(&event);
        }
    }

    pub fn update(
        &mut self, delta: f32, editor: &mut MapEditor, input: &mut InputModel
    ) {
        // Update the UI
        self.ui.update(delta, editor);
        input.cursor_over_ui = self.ui.cursor_over_ui();

        // Update the viewport
        self.viewport.update(delta, editor, input, &self.renderer, &mut self.window);
    }

    pub fn render(&mut self) -> Result<(), Error> {
        // Render everything
        let mut frame = self.window_renderer.start_frame(&mut self.renderer);
        self.viewport.render(
            &mut frame,
            &mut self.renderer, &mut self.window_renderer,
            &mut self.world3d_renderer, &mut self.world3d_rendertarget,
        );
        self.ui.render(
            &mut frame,
            &mut self.renderer, &mut self.window_renderer,
            &mut self.simple2d_renderer, &mut self.simple2d_rendertarget,
        )?;
        self.window_renderer.finish_frame(&mut self.renderer, frame);
        self.window.swap_buffers();

        Ok(())
    }
}
