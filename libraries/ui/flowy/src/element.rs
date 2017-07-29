use cgmath::{Vector2, Point2};
use screenmath::{Rectangle, Lrtb};
use rusttype::{Point, GlyphId, Rect, point, Font};
use glyphlayout::{self, AlignH, AlignV};

use style::{Style, FlowDirection, Position, SideH, SideV};

pub struct Element {
    // TODO: Use this value to check against stale index IDs
    pub(crate) inner_id: i32,

    pub style: Style,
    pub mode: ElementMode,
    pub text: Option<ElementText>,

    // Cache data
    pub(crate) positioning: Positioning,

    // Input state
    pub(crate) cursor_state: ElementCursorState,
    pub(crate) clicked: bool,
    pub(crate) focused: bool,
}

impl Element {
    pub fn new(style: Style) -> Self {
        Element {
            inner_id: -1,

            style,
            mode: ElementMode::Passive,
            text: None,

            positioning: Positioning::new(),

            cursor_state: ElementCursorState::None,
            clicked: false,
            focused: false,
        }
    }

    pub fn with_text<S: Into<String>>(text: S, style: Style) -> Self {
        let mut element = Self::new(style);
        element.set_text(text);
        element
    }

    pub fn positioning(&self) -> &Positioning {
        &self.positioning
    }

    pub fn cursor_state(&self) -> ElementCursorState {
        self.cursor_state
    }

    pub fn hovering(&self) -> bool {
        self.cursor_state == ElementCursorState::Hovering
    }

    pub fn held(&self) -> bool {
        self.cursor_state == ElementCursorState::Held
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    /// Retrieves the text from the inner text structure, or returns an empty string.
    pub fn text(&self) -> &str {
        if let Some(ref element_text) = self.text {
            element_text.text()
        } else {
            ""
        }
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        let text = text.into();

        if let Some(ref mut element_text) = self.text {
            element_text.set_text(text);
            return;
        }

        self.text = Some(ElementText::new(text));
    }

    pub fn update_layout(
        &mut self,
        parent_container: &Rectangle<f32>, parent_padding: &Lrtb,
        flow_cursor: &mut Point2<f32>, flow_margin: &mut f32, flow_direction: FlowDirection,
        fonts: &Vec<Font>,
    ) {
        self.update_positioning(
            parent_container, parent_padding, flow_cursor, flow_margin, flow_direction
        );

        if let Some(ref mut text) = self.text {
            text.update_glyphs(&self.positioning.container, &self.style, fonts);
        }
    }

    fn update_positioning(
        &mut self,
        parent_container: &Rectangle<f32>, parent_padding: &Lrtb,
        flow_cursor: &mut Point2<f32>, flow_margin: &mut f32, flow_direction: FlowDirection,
    ) {
        let style = &self.style;

        // Calculate the final size of this element, it's needed for some positioning types
        let parent_size = parent_container.size();
        let size = style.size.to_units(parent_size, parent_padding);

        // Calculate the base position of this element
        let marginless_position = match &style.position {
            &Position::Flow => flow_direction.position(*flow_cursor, size),
            &Position::Relative(position, dock_h, dock_v) => {
                // Calculate the position based on our size, the container, and the docking
                parent_container.min + Vector2::new(
                    dock_h.relative_position(
                        position.x, size.x,
                        parent_size.x - parent_padding.left - parent_padding.right
                    ),
                    dock_v.relative_position(
                        position.y, size.y,
                        parent_size.y - parent_padding.top - parent_padding.bottom
                    ),
                ) + parent_padding.left_top()
            },
        };

        // Add margins to that base position if we're in flow mode, merging margins
        let position = if style.position.is_flow() {
            // TODO: This doesn't take flow direction into account and assumes Right
            marginless_position + style.margin.max_left(*flow_margin).left_top()
        } else {
            marginless_position
        };

        // If we're positioned using flow, adjust the flow position
        if style.position.is_flow() {
            *flow_cursor = flow_direction.advance_cursor(position, size, *flow_cursor);
            // TODO: This doesn't take flow direction into account and assumes Right
            *flow_margin = style.margin.right;
        }

        // Store the calculated data
        self.positioning = Positioning {
            container: Rectangle::start_size(position, size),
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementMode {
    /// Does not respond to mouse input.
    Passive,
    /// Blocks mouse input, detects click events, uses hover and active styling.
    Clickable,
    /// On click, will be focused and receive text input.
    TextField,
}

#[derive(Debug)]
pub struct Positioning {
    pub container: Rectangle<f32>,
}

impl Positioning {
    pub fn new() -> Self {
        Positioning {
            container: Rectangle::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementCursorState {
    None,
    Hovering,
    Held,
}

#[derive(Debug)]
pub struct ElementText {
    text: String,
    glyphs: Option<Vec<(GlyphId, Point<f32>)>>,

    // This is stuff for the renderer to touch
    pub cache_stale: bool,
    // TODO: Previously the renderer would use this rect to know when to reposition glyphs, but the
    // new layout glyphs system doesn't use it, make sure it moves glyphs when needed.
    pub cache_rect: Rectangle<f32>,
}

impl ElementText {
    pub fn new(text: String) -> Self {
        ElementText {
            text: text,
            glyphs: None,

            cache_stale: true,
            cache_rect: Rectangle::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    /// Marks the cache data as stale.
    pub fn text_mut(&mut self) -> &mut String {
        self.glyphs = None;
        self.cache_stale = true;
        &mut self.text
    }

    /// Marks the cache data as stale.
    pub fn set_text(&mut self, text: String) {
        if text != self.text {
            self.text = text;
            self.glyphs = None;
            self.cache_stale = true;
        }
    }

    pub fn cached_glyphs(&self) -> Option<&Vec<(GlyphId, Point<f32>)>> {
        self.glyphs.as_ref()
    }

    pub fn update_glyphs(&mut self, container: &Rectangle<f32>, style: &Style, fonts: &Vec<Font>) {
        if self.glyphs.is_some() {
            return;
        }

        // Layout the text
        let align_h = match style.text_align.0 {
            SideH::Left => AlignH::Left,
            SideH::Center => AlignH::Center,
            SideH::Right => AlignH::Right,
        };
        let align_v = match style.text_align.1 {
            SideV::Top => AlignV::Top,
            SideV::Center => AlignV::Center,
            SideV::Bottom => AlignV::Bottom,
        };
        let font = fonts.get(style.text_font.0).expect("Unable to find font on element");
        let glyphs = glyphlayout::layout_text(
            &self.text, font, style.text_size,
            Rect {
                min: point(container.min.x + style.padding.left, container.min.y + style.padding.top),
                max: point(container.max.x - style.padding.right, container.max.y - style.padding.bottom),
            }, (align_h, align_v),
        );

        // Extract just the data we need
        let mut cached_glyphs = Vec::new();
        for glyph in glyphs {
            cached_glyphs.push((glyph.id(), glyph.position()));
        }
        self.glyphs = Some(cached_glyphs);
    }
}
