use cgmath::{Vector2};
use input::{Input};
use palette::pixel::{Srgb};

use calcium_game::{AverageDelta, delta_to_fps};
use calcium_rendering::{Renderer, WindowRenderer, Error};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer};
use calcium_ui::{UiRenderer, Ui, Element, ElementId, widget};
use calcium_ui::style::{Style, Position, Size, SideH, SideV};

use model::{MapEditorModel};

pub struct UiView<R: Renderer> {
    ui_renderer: UiRenderer<R>,

    ui: Ui,
    new_brush_button_id: ElementId,
    fps_id: ElementId,

    average_delta: AverageDelta,
}

impl<R: Renderer> UiView<R> {
    pub fn new(renderer: &mut R) -> Result<Self, Error> {
        let ui_renderer = UiRenderer::new(renderer)?;

        let mut ui = Ui::new();
        let root_id = ui.root_id();

        // Create the top ribbon
        let ribbon_buttons_id = widget::ribbon(&mut ui, root_id);

        // Add a few buttons to the top ribbon
        let _ = widget::ribbon_buton("Save As", &mut ui, ribbon_buttons_id);
        let _ = widget::ribbon_buton("Load", &mut ui, ribbon_buttons_id);
        let new_brush_button_id = widget::ribbon_buton("New Brush", &mut ui, ribbon_buttons_id);

        // Add a FPS label
        let fps = Element::new(Style {
            position: Position::Relative(Vector2::new(0.0, 0.0), SideH::Right, SideV::Top),
            size: Size::units(120.0, 14.0),
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            text_size: 14.0,
            .. Style::new()
        });
        let fps_id = ui.add_child(fps, root_id);

        Ok(UiView {
            ui_renderer,

            ui,
            new_brush_button_id,
            fps_id,

            average_delta: AverageDelta::new(),
        })
    }

    pub fn handle_event(&mut self, event: &Input) {
        self.ui.handle_event(event);
    }

    pub fn update(&mut self, delta: f32, editor: &mut MapEditorModel) {
        self.ui.process_input_frame();
        self.average_delta.accumulate(delta);

        {
            let button = &mut self.ui[self.new_brush_button_id];
            if button.clicked() {
                editor.new_brush();
            }
        }

        {
            let fps = &mut self.ui[self.fps_id];
            fps.set_text(
                format!("FPS: {}", delta_to_fps(self.average_delta.get()))
            );
        }
    }

    pub fn render<SR: Simple2DRenderer<R>>(
        &mut self, frame: &mut R::Frame,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer,
        simple2d_renderer: &mut SR,
        simple2d_rendertarget: &mut Simple2DRenderTarget<R, SR>,
    ) -> Result<(), Error> {
        let ui_batches = self.ui_renderer.draw(
            &mut self.ui, window_renderer.size().cast(), renderer
        )?;

        simple2d_renderer.render(
            &ui_batches, simple2d_rendertarget,
            renderer, window_renderer, frame
        );

        Ok(())
    }

    pub fn cursor_over_ui(&self) -> bool {
        self.ui.cursor_active_element().is_some()
    }
}
