use std::ops::{Index, IndexMut};

use calcium_rendering_simple2d::{Rectangle};
use cgmath::{Vector2, Zero};
use input::{Input, Motion, Button, MouseButton, Key};

use style::{Style, Size, Position, FlowDirection, Lrtb};
use element::{Positioning};
use {Element, ElementCursorState, ElementMode};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(usize);

pub struct Ui {
    elements: Vec<Option<Element>>,
    child_connections: Vec<(ElementId, Vec<ElementId>)>, // Parent, then children
    next_inner_id: i32,

    cursor_position: Vector2<f32>,
    cursor_active_element: Option<usize>,
    text_active_element: Option<usize>,

    // pressed/released are reset every frame, state is persistent
    cursor_pressed: bool,
    cursor_released: bool,
    cursor_state: bool,
}

impl Ui {
    pub fn new() -> Self {
        let mut root = Element::new(Style::new());
        root.inner_id = 0;

        // The UI should already include a root
        Ui {
            elements: vec!(Some(root)),
            child_connections: vec!((ElementId(0), Vec::new())),
            next_inner_id: 1,

            cursor_position: Vector2::new(0.0, 0.0),
            cursor_active_element: None,
            text_active_element: None,

            cursor_pressed: false,
            cursor_released: false,
            cursor_state: false,
        }
    }

    pub fn root_id(&self) -> ElementId {
        ElementId(0)
    }

    pub fn get(&self, id: ElementId) -> Option<&Element> {
        self.elements.get(id.0)
            .and_then(|e| e.as_ref())
    }

    pub fn get_mut(&mut self, id: ElementId) -> Option<&mut Element> {
        self.elements.get_mut(id.0)
            .and_then(|e| e.as_mut())
    }

    pub fn cursor_active_element(&self) -> Option<ElementId> {
        self.cursor_active_element.map(|v| ElementId(v))
    }

    pub fn parent_of(&self, child: ElementId) -> ElementId {
        self.child_connections[child.0].0
    }

    pub fn children_of(&self, parent: ElementId) -> &Vec<ElementId> {
        &self.child_connections[parent.0].1
    }

    pub fn add_child(&mut self, mut child: Element, parent: ElementId) -> ElementId {
        // Make sure this element gets an inner ID
        // TODO: Not currently use, use for preventing stale index IDs by adding an extra check
        child.inner_id = self.next_inner_id;
        self.next_inner_id += 1;

        // Check if we can find an empty slot
        if let Some((index, slot)) = self.elements.iter_mut()
            .enumerate().find(|v| v.1.is_none()) {
            // We found an empty slot, fill it
            *slot = Some(child);
            let child_id = ElementId(index);

            // Since this is an existing slot, we already have a (cleared) child connections list,
            // we only need to set the parent
            self.child_connections[parent.0].1.push(child_id);

            return child_id
        }

        // We didn't find an empty slot, add a new one at the end
        self.elements.push(Some(child));
        let child_id = ElementId(self.elements.len() - 1);

        // Add the child connections for this element as well
        self.child_connections.push((parent, Vec::new()));
        self.child_connections[parent.0].1.push(child_id);

        child_id
    }

    pub fn remove(&mut self, id: ElementId) -> ElementId {
        // First, remove the element and replace the dependencies vector for it
        self.elements[id.0] = None;
        let children = ::std::mem::replace(
            &mut self.child_connections[id.0],
            (ElementId(0), Vec::new())
        );

        // Remove the element from its parent, this may just do nothing if it's an orphan element
        self.child_connections[(children.0).0].1.retain(|v| id != *v);

        // Now go through all the children of the element and do the same thing recursively
        for child in children.1 {
            self.remove(child);
        }

        ElementId(0)
    }

    pub fn handle_event(&mut self, event: &Input) {
        // In case we have text input
        let elements = &mut self.elements;
        let el_text = self.text_active_element
            .and_then(|id| elements[id].as_mut())
            .and_then(|element| element.text.as_mut());

        match *event {
            Input::Press(Button::Mouse(MouseButton::Left)) => {
                self.cursor_pressed = true;
                self.cursor_released = false;
                self.cursor_state = true;
            },
            Input::Release(Button::Mouse(MouseButton::Left)) => {
                self.cursor_pressed = false;
                self.cursor_released = true;
                self.cursor_state = false;
            },
            Input::Move(Motion::MouseCursor(x, y)) =>
                self.cursor_position = Vector2::new(x, y).cast(),
            Input::Text(ref text) => {
                // We received text, so pass it to the element
                if let Some(el_text) = el_text {
                    el_text.text.push_str(text);
                    el_text.cache_stale = true;
                }
            },
            Input::Press(Button::Keyboard(Key::Backspace)) => {
                // We received a backspace, remove text from the element
                if let Some(el_text) = el_text {
                    el_text.text.pop();
                    el_text.cache_stale = true;
                }
            },
            _ => {}
        }
    }

    /// Processes input gathered through handle_event and updates event values on elements.
    pub fn process_input_frame(&mut self) {
        // Reset the previous input frame
        if let Some(id) = self.cursor_active_element.take() {
            if let Some(ref mut element) = self.elements[id] {
                element.cursor_state = ElementCursorState::None;
                element.clicked = false;
            }
        }

        // If we clicked, clear the previous active text field element, so when we click outside
        // with one focused, it gets un-focused
        if self.cursor_released {
            if let Some(id) = self.text_active_element.take() {
                if let Some(ref mut element) = self.elements[id] {
                    element.focused = false;
                }
            }
        }

        // Go through all elements and see if the mouse is over any of them
        for id in 0..self.elements.len() {
            if let Some(ref element) = self.elements[id] {
                // Make sure this element actually captures mouse input
                if element.mode == ElementMode::Passive {
                    continue;
                }

                // Check if the mouse is over this and if so set it to hovering
                // TODO: Make use of a layering value calculated during calculate_positioning
                if element.positioning.rectangle.contains(self.cursor_position) {
                    // Remember this element so we can update it later if it's indeed on top
                    self.cursor_active_element = Some(id);
                }
            }
        }

        // If anything became active again, mark it as such
        if let Some(id) = self.cursor_active_element {
            let element = self.elements[id].as_mut().unwrap();
            element.cursor_state = if self.cursor_state {
                ElementCursorState::Hovering
            } else {
                ElementCursorState::Held
            };

            // Check if the cursor was released over this element so we can raise a click
            // TODO: The expected behavior is to keep track of which element the click was
            //  started on and raise the clicked event regardless of where it ended.
            if self.cursor_released {
                element.clicked = true;

                // If the element clicked on was a text field, set it as the active text element so
                // it can be rendered focused and receive input
                if element.mode == ElementMode::TextField {
                    element.focused = true;
                    self.text_active_element = Some(id);
                }
            }
        }

        // Reset cursor pressed data
        self.cursor_pressed = false;
        self.cursor_released = false;
    }

    pub fn calculate_positioning(&mut self, viewport_size: Vector2<f32>) {
        let root_id = self.root_id();

        // Lock the root to match the viewport
        {
            let style = &mut self[root_id].style;
            style.size = Size::units(viewport_size.x, viewport_size.y);
        }

        // Start off the calculation at the root
        self.calculate_element_positioning(
            root_id,
            &Rectangle::new(Vector2::zero(), viewport_size), &Lrtb::uniform(0.0),
            &mut Vector2::new(0.0, 0.0), &mut 0.0, FlowDirection::Right,
        );
    }

    pub fn calculate_element_positioning(
        &mut self, element_id: ElementId,
        parent_container: &Rectangle<f32>, parent_padding: &Lrtb,
        flow_cursor: &mut Vector2<f32>, flow_margin: &mut f32, flow_direction: FlowDirection,
    ) {
        let size;
        let our_container;
        let our_padding;
        let mut child_flow_cursor;
        let mut child_flow_margin;
        let child_flow_direction;

        {
            let element = &mut self[element_id];
            let style = &element.style;

            // Calculate the final size of this element, it's needed for some positioning types
            let parent_size = parent_container.size();
            size = style.size.to_units(parent_size, parent_padding);

            // Calculate the base position of this element
            let marginless_position = match &style.position {
                &Position::Flow => flow_direction.position(*flow_cursor, size),
                &Position::Relative(position, dock_h, dock_v) => {
                    // Calculate the position based on our size, the container, and the docking
                    Vector2::new(
                        dock_h.relative_position(
                            position.x, size.x,
                            parent_size.x - parent_padding.left - parent_padding.right
                        ),
                        dock_v.relative_position(
                            position.y, size.y,
                            parent_size.y - parent_padding.top - parent_padding.bottom
                        ),
                    ) + parent_container.start + parent_padding.left_top()
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
            element.positioning = Positioning {
                rectangle: Rectangle::start_size(position, size),
            };

            // Calculate the flow data needed by the children based on this element's flow data
            our_container = Rectangle::start_size(position, size);
            our_padding = element.style.padding.clone();
            child_flow_direction = element.style.flow_direction;
            child_flow_margin = element.style.padding.left;
            child_flow_cursor = child_flow_direction.flow_start(&our_container);
        }

        // Now go through all the children as well
        for child_id in self.children_of(element_id).clone() {
            self.calculate_element_positioning(
                child_id, &our_container, &our_padding,
                &mut child_flow_cursor, &mut child_flow_margin, child_flow_direction,
            );
        }
    }
}

impl Index<ElementId> for Ui {
    type Output = Element;

    fn index<'a>(&'a self, index: ElementId) -> &'a Element {
        self.get(index).expect("Unable to find element")
    }
}

impl IndexMut<ElementId> for Ui {
    fn index_mut<'a>(&'a mut self, index: ElementId) -> &'a mut Element {
        self.get_mut(index).expect("Unable to find element")
    }
}
