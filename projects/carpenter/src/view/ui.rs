use cgmath::{Vector2};
use conrod::{self, color, Widget, Positionable, Sizeable, Ui, UiBuilder, Colorable};
use conrod::text::{FontCollection};
use conrod::widget::{Text, Canvas, Button};
use input::{Input};

use calcium_game::{AverageDelta, delta_to_fps};
use calcium_rendering::{Renderer, WindowRenderer, Error};
use calcium_rendering_simple2d::{RenderBatch, Simple2DRenderTarget, Simple2DRenderer};
use calcium_conrod::{ConrodRenderer};

use model::{MapEditorModel};

pub struct UiView<R: Renderer> {
    conrod_renderer: ConrodRenderer<R>,
    ui_batches: Vec<RenderBatch<R>>,

    ui: Ui,
    ids: Ids,

    average_delta: AverageDelta,
}

impl<R: Renderer> UiView<R> {
    pub fn new(size: Vector2<u32>, renderer: &mut R) -> Result<Self, Error> {
        let conrod_renderer = ConrodRenderer::new(renderer)?;
        let ui_batches = vec!();

        let mut ui = UiBuilder::new(size.cast().into())
            .theme(theme())
            .build();
        ui.fonts.insert(FontCollection::from_bytes(::ttf_noto_sans::REGULAR).into_font().unwrap());
        let ids = Ids::new(ui.widget_id_generator());

        Ok(UiView {
            conrod_renderer,
            ui_batches,

            ui,
            ids,

            average_delta: AverageDelta::new(),
        })
    }

    pub fn handle_event(&mut self, event: &Input, window_renderer: &R::WindowRenderer) {
        let size = window_renderer.size();
        if let Some(event) = ::conrod::backend::piston::event::convert(
            event.clone(), size.x as f64, size.y as f64
        ) {
            self.ui.handle_event(event);
        }
    }

    pub fn update(&mut self, delta: f32, editor: &mut MapEditorModel) {
        let ui = &mut self.ui.set_widgets();
        self.average_delta.accumulate(delta);

        // Root canvas
        Canvas::new()
            .color(color::TRANSPARENT)
            .set(self.ids.canvas, ui);

        // Top ribbon
        Canvas::new()
            .top_left_of(self.ids.canvas)
            .h(108.0)
            .pad(3.0)
            .set(self.ids.ribbon_canvas, ui);

        // Save button
        Button::new()
            .up_from(self.ids.ribbon_save_label, 3.0)
            .w_h(60.0, 60.0)
            .set(self.ids.ribbon_save, ui);
        Text::new("Save As")
            .bottom_left_of(self.ids.ribbon_canvas)
            .w_h(60.0, 12.0)
            .font_size(10)
            .center_justify()
            .set(self.ids.ribbon_save_label, ui);

        // Load button
        Button::new()
            .up_from(self.ids.ribbon_load_label, 3.0)
            .w_h(60.0, 60.0)
            .set(self.ids.ribbon_load, ui);
        Text::new("Load")
            .right_from(self.ids.ribbon_save_label, 3.0)
            .w_h(60.0, 12.0)
            .font_size(10)
            .center_justify()
            .set(self.ids.ribbon_load_label, ui);

        // New brush button
        for _click in Button::new()
            .up_from(self.ids.ribbon_new_brush_label, 3.0)
            .w_h(60.0, 60.0)
            .set(self.ids.ribbon_new_brush, ui) {
            editor.new_brush();
        }
        Text::new("New Brush")
            .right_from(self.ids.ribbon_load_label, 3.0)
            .w_h(60.0, 12.0)
            .font_size(10)
            .center_justify()
            .set(self.ids.ribbon_new_brush_label, ui);

        // Render performance debug information
        Text::new(&format!("FPS: {}", delta_to_fps(self.average_delta.get())))
            .top_right_of(self.ids.ribbon_canvas)
            .w(96.0)
            .font_size(12)
            .set(self.ids.fps_label, ui);
        Text::new(&format!("MS: {}", self.average_delta.get()))
            .left_from(self.ids.fps_label, 12.0)
            .w(96.0)
            .font_size(12)
            .set(self.ids.ms_label, ui);
    }

    pub fn render<SR: Simple2DRenderer<R>>(
        &mut self, frame: &mut R::Frame,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer,
        simple2d_renderer: &mut SR,
        simple2d_rendertarget: &mut Simple2DRenderTarget<R, SR>,
    ) -> Result<(), Error> {
        // Create render batches for the UI
        if let Some(changed_batches) = self.conrod_renderer.draw_if_changed(
            renderer, window_renderer, &mut self.ui
        )? {
            self.ui_batches = changed_batches;
        }

        simple2d_renderer.render(
            &self.ui_batches, simple2d_rendertarget,
            renderer, window_renderer, frame
        );

        Ok(())
    }

    pub fn cursor_over_ui(&self) -> bool {
        let widget = self.ui.global_input().current.widget_under_mouse;
        widget
            // If we're over a widget, pass through the background canvas
            .map(|w| w != self.ids.canvas)
            // If there no widget, we're not over ui either way
            .unwrap_or(false)
    }
}

fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: color::DARK_CHARCOAL,
        shape_color: color::LIGHT_CHARCOAL,
        border_color: color::BLACK,
        border_width: 0.0,
        label_color: color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: ::std::time::Duration::from_millis(500),
    }
}

widget_ids! { struct Ids {
    canvas,

    ribbon_canvas,
    ribbon_save, ribbon_save_label,
    ribbon_load, ribbon_load_label,
    ribbon_new_brush, ribbon_new_brush_label,

    fps_label, ms_label,
    counter, text_field,
} }
