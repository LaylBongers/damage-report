use calcium_rendering::{Types, Error, WindowRenderer};
use calcium_rendering_simple2d::{Simple2DTypes, Simple2DRenderTarget};
use calcium_rendering_world3d::{World3DTypes, World3DRenderTarget};
use calcium_rendering_static::{Initializer};
use window::{Window, AdvancedWindow, WindowSettings};
use slog::{Logger};

use model::{MapEditorModel, InputModel};
use view::{ViewportView, UiView};

pub struct WindowView<W: Window, T: Types, ST: Simple2DTypes<T>, WT: World3DTypes<T>> {
    window: W,
    renderer: T::Renderer,
    window_renderer: T::WindowRenderer,

    simple2d_renderer: ST::Renderer,
    simple2d_rendertarget: Simple2DRenderTarget<T, ST>,

    world3d_renderer: WT::Renderer,
    world3d_rendertarget: World3DRenderTarget<T, WT>,

    viewport: ViewportView<T, WT>,
    ui: UiView<T>,
}

impl<W: Window + AdvancedWindow, T: Types, ST: Simple2DTypes<T>, WT: World3DTypes<T>>
    WindowView<W, T, ST, WT> {
    pub fn new<I: Initializer<Window=W, Types=T, World3DTypes=WT, Simple2DTypes=ST>>(
        log: &Logger,
        init: &I,
        editor: &mut MapEditorModel,
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
        let ui = UiView::new(window_renderer.size(), &mut renderer)?;

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

    pub fn handle_events<I: Initializer<Window=W, Types=T, World3DTypes=WT, Simple2DTypes=ST>>(
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
            self.ui.handle_event(&event, &self.window_renderer);
        }
    }

    pub fn update(&mut self, delta: f32, input: &mut InputModel, editor: &mut MapEditorModel) {
        // Update the UI
        self.ui.update(delta, editor);
        input.cursor_over_ui = self.ui.cursor_over_ui();

        // Update the viewport
        self.viewport.update(delta, &input, &mut self.window);
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
