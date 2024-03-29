use cgmath::{Vector2, Point2};
use screenmath::{Rectangle, Lrtb};
use rusttype::{Point, GlyphId, Rect, point, Font};
use glyphlayout::{self, AlignH, AlignV};

use style::{Style, FlowDirection, Position, SideH, SideV};

/// The primitive UI element all UI widgets are built up from.
pub struct Element {
    // TODO: Use this value to check against stale index IDs
    pub(crate) inner_id: i32,

    style: Style,
    behavior: ElementBehavior,

    // Cache data
    text_internal: Option<ElementText>,
    positioning: Positioning,

    // Input state
    pub(crate) cursor_state: ElementCursorState,
    pub(crate) clicked: bool,
    pub(crate) focused: bool,
}

impl Element {
    /// Creates a new element with the given style.
    pub fn new(style: Style) -> Self {
        Element {
            inner_id: -1,

            style,
            behavior: ElementBehavior::Passive,

            text_internal: None,
            positioning: Positioning::new(),

            cursor_state: ElementCursorState::None,
            clicked: false,
            focused: false,
        }
    }

    /// Gets the style, which defines how the element looks on screen.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Gets the style as mutable.
    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    /// Sets the style.
    pub fn set_style(&mut self, value: Style) {
        self.style = value;
    }

    /// Gets the behavior, which defines how the element responds to input.
    pub fn behavior(&self) -> &ElementBehavior {
        &self.behavior
    }

    /// Gets the behavior as mutable.
    pub fn behavior_mut(&mut self) -> &mut ElementBehavior {
        &mut self.behavior
    }

    /// Sets the behavior.
    pub fn set_behavior(&mut self, value: ElementBehavior) {
        self.behavior = value;
    }

    /// Retrieves the text from the internal text data, or returns an empty string.
    pub fn text(&self) -> &str {
        if let Some(ref element_text) = self.text_internal {
            element_text.text()
        } else {
            ""
        }
    }

    /// Sets the text content of this element.
    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        let text = text.into();

        if let Some(ref mut element_text) = self.text_internal {
            element_text.set_text(text);
            return;
        }

        self.text_internal = Some(ElementText::new(text));
    }

    /// Gets the internal text structure, which contains caching data.
    pub fn text_internal(&self) -> &Option<ElementText> {
        &self.text_internal
    }

    /// Gets the internal text structure as mutable, which contains caching data.
    pub fn text_internal_mut(&mut self) -> &mut Option<ElementText> {
        &mut self.text_internal
    }

    pub fn positioning(&self) -> &Positioning {
        &self.positioning
    }

    pub fn positioning_mut(&mut self) -> &mut Positioning {
        &mut self.positioning
    }

    pub fn cursor_state(&self) -> ElementCursorState {
        self.cursor_state
    }

    /// Return strue if the mouse cursor is currently hovering over this element.
    pub fn hovering(&self) -> bool {
        self.cursor_state == ElementCursorState::Hovering
    }

    /// Returns true if this element is currently being held with the left mouse button.
    pub fn held(&self) -> bool {
        self.cursor_state == ElementCursorState::Held
    }

    /// Returns true if this element was clicked in the last UI frame.
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Returns true if this element is currently focused. Relevant for elements that can be
    /// 'focused', meaning they can be selected for input. Elements can be tabbed between to be
    /// focused, buttons can be clicked with enter while focused, text fields can receive text
    /// input only while focused.
    ///
    /// TODO: Tabbing between elements is not yet implemented, only clicking on text fields
    /// currently focuses them.
    pub fn focused(&self) -> bool {
        self.focused
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

        if let Some(ref mut text) = self.text_internal {
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

        let positioning = Positioning {
            container: Rectangle::start_size(position, size),
        };

        // Check if the positioning is different, if it is we need to invalidate the text
        // TODO: Re-position glyphs instead
        if self.positioning != positioning {
            if let Some(ref mut text) = self.text_internal {
                text.glyphs = None;
                text.cache_stale = true;
            }
        }

        // Store the calculated data
        self.positioning = positioning;
    }
}

/// The type of input an element takes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementBehavior {
    /// Does not respond to mouse input.
    Passive,
    /// Blocks mouse input, detects click events, uses hover and active styling.
    Clickable,
    /// On click, will be focused and receive text input.
    TextField,
}

/// Cached layout positioning data for an element.
#[derive(Debug, PartialEq)]
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

/// The current cursor input state of an element.
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
    // TODO: Investigate if this is still used and if so how it's used
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
