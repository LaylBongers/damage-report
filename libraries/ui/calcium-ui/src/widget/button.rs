use palette::pixel::{Srgb};

use style::{Style, CursorBehavior, Size, SideH, SideV, Lrtb};
use {Ui, Element, ElementId};

pub fn button(label: &str, parent_id: ElementId, ui: &mut Ui) -> ElementId {
    let button_color = Srgb::new(0.53, 0.54, 0.52).into();
    let button_style = Style {
        size: Size::units(90.0, 24.0),
        margin: Lrtb::uniform(6.0),
        background_color: Some(button_color),
        text_size: 18.0,
        text_color: Srgb::new(1.0, 1.0, 1.0).into(),
        text_align: (SideH::Center, SideV::Center),
        cursor_behavior: CursorBehavior::clickable_autocolor(button_color),
        .. Style::new()
    };

    let mut submit_button = Element::new(button_style.clone());
    submit_button.set_text(label);
    ui.add_child(submit_button, parent_id)
}
