use cgmath::{Vector2};
use conrod::{self, color, Widget, Positionable, Sizeable, Labelable, Ui, UiBuilder, Colorable};
use conrod::text::{FontCollection};
use conrod::widget::{self, Text, Canvas, Button};

use calcium_game::{AverageDelta, delta_to_fps};

pub struct EditorUi {
    pub ui: Ui,
    ids: Ids,

    average_delta: AverageDelta,
    count: i32,
    text: String,
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
            count: 0,
            text: String::from("Data"),
        }
    }

    pub fn update(&mut self, delta: f32) {
        let ui = &mut self.ui.set_widgets();
        self.average_delta.accumulate(delta);

        // The root canvas
        Canvas::new()
            .color(color::TRANSPARENT)
            .set(self.ids.canvas, ui);

        // FPS label
        Text::new(&format!("FPS: {}", delta_to_fps(self.average_delta.get())))
            .top_right_of(self.ids.canvas)
            .w(100.0)
            .font_size(12)
            .set(self.ids.fps_label, ui);
        Text::new(&format!("MS: {}", self.average_delta.get()))
            .left_from(self.ids.fps_label, 12.0)
            .w(100.0)
            .font_size(12)
            .set(self.ids.ms_label, ui);

        // Counter button
        for _click in Button::new()
            .middle_of(self.ids.canvas)
            .w_h(240.0, 80.0)
            .label(&self.count.to_string())
            .set(self.ids.counter, ui)
        {
            self.count += 1;
        }

        for event in widget::TextBox::new(&self.text)
            .down_from(self.ids.counter, 12.0)
            .font_size(18)
            .w_h(240.0, 36.0)
            .set(self.ids.text_field, ui)
        {
            match event {
                widget::text_box::Event::Enter => println!("TextBox {}", self.text),
                widget::text_box::Event::Update(string) => self.text = string,
            }
        }
    }
}

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
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
    fps_label, ms_label,
    counter, text_field,
} }
