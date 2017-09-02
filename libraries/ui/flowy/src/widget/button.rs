use palette::pixel::{Srgb};
use screenmath::{Lrtb};

use style::{Style, Size, SideH, SideV, color_highlight, color_active};
use {Ui, Element, ElementId, ElementBehavior};

/// A clickable button.
pub fn button(label: &str, parent_id: ElementId, ui: &mut Ui) -> ElementId {
    let button_color = Srgb::new(0.53, 0.54, 0.52).into();
    let button_style = Style {
        size: Size::units(90.0, 24.0),
        margin: Lrtb::uniform(6.0),

        background_color: Some(button_color),
        hover_color: Some(color_highlight(button_color)),
        active_color: Some(color_active(button_color)),

        text_size: 18.0,
        text_color: Srgb::new(1.0, 1.0, 1.0).into(),
        text_align: (SideH::Center, SideV::Center),
        .. Style::new()
    };

    let mut submit_button = Element::new(button_style.clone());
    submit_button.set_text(label);
    submit_button.set_behavior(ElementBehavior::Clickable);
    ui.elements.add_child(submit_button, parent_id)
}
