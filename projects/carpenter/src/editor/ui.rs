use cgmath::{Vector2};
use conrod::{self, color, Widget, Positionable, Sizeable, Labelable, Ui, UiBuilder, Colorable};
use conrod::text::{FontCollection};
use conrod::widget::{Text, Canvas, Button};

use calcium_game::{AverageDelta, delta_to_fps};

pub struct EditorUi {
    pub ui: Ui,
    ids: Ids,

    average_delta: AverageDelta,
}

impl EditorUi {
    pub fn new(size: Vector2<u32>) -> Self {
        let mut ui = UiBuilder::new(size.cast().into())
            .theme(theme())
            .build();
        ui.fonts.insert(FontCollection::from_bytes(::ttf_noto_sans::REGULAR).into_font().unwrap());
        let ids = Ids::new(ui.widget_id_generator());

        EditorUi {
            ui,
            ids,

            average_delta: AverageDelta::new(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        let ui = &mut self.ui.set_widgets();
        self.average_delta.accumulate(delta);

        // Root canvas
        Canvas::new()
            .color(color::TRANSPARENT)
            .set(self.ids.canvas, ui);

        // Top ribbon
        Canvas::new()
            .top_left_of(self.ids.canvas)
            .h(36.0) // Eventual full size: .h(96.0)
            .pad(3.0)
            .set(self.ids.top_ribbon_canvas, ui);

        // Save button
        Button::new()
            .top_left_of(self.ids.top_ribbon_canvas)
            .w_h(30.0, 30.0)
            .label("S")
            .set(self.ids.top_ribbon_save, ui);

        // Load button
        Button::new()
            .right_from(self.ids.top_ribbon_save, 3.0)
            .w_h(30.0, 30.0)
            .label("L")
            .set(self.ids.top_ribbon_load, ui);

        // Render performance debug information
        Text::new(&format!("FPS: {}", delta_to_fps(self.average_delta.get())))
            .top_right_of(self.ids.top_ribbon_canvas)
            .w(96.0)
            .font_size(12)
            .set(self.ids.fps_label, ui);
        Text::new(&format!("MS: {}", self.average_delta.get()))
            .left_from(self.ids.fps_label, 12.0)
            .w(96.0)
            .font_size(12)
            .set(self.ids.ms_label, ui);
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

    top_ribbon_canvas,
    top_ribbon_save, top_ribbon_load,

    fps_label, ms_label,
    counter, text_field,
} }
