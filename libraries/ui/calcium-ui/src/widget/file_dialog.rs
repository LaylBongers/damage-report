use cgmath::{Vector2};
use palette::pixel::{Srgb};
use style::{Style, CursorBehavior, Size, Position, SideH, SideV};
use {Ui, Element, ElementId};

pub struct FileDialog {
    _shade_id: ElementId,
}

impl FileDialog {
    pub fn new(ui: &mut Ui, root_id: ElementId) -> Self {
        // Transparent shade that blocks off the background, making this a modal dialog
        let shade = Element::new(Style {
            position: Position::Relative(Vector2::new(0.0, 0.0), SideH::Left, SideV::Top),
            size: Size::scale(1.0, 1.0),
            background_color: Some(Srgb::with_alpha(0.1, 0.1, 0.1, 0.5).into()),
            cursor_behavior: CursorBehavior::Block,
            .. Style::new()
        });
        let shade_id = ui.add_child(shade, root_id);

        FileDialog {
            _shade_id: shade_id,
        }
    }
}
