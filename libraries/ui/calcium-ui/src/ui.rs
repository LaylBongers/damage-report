use std::ops::{Index, IndexMut};

use calcium_rendering_simple2d::{Rectangle};
use cgmath::{Vector2};
use input::{Input, Motion, Button, MouseButton};

use style::{Style, Size, Position, CursorBehavior};
use element::{Positioning};
use {Element, ElementCursorState};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(usize);

pub struct Ui {
    elements: Vec<Element>,
    child_connections: Vec<Vec<ElementId>>,

    cursor_position: Vector2<f32>,
    // pressed/released are reset every frame, state is persistent
    cursor_pressed: bool,
    cursor_released: bool,
    cursor_state: bool,
}

impl Ui {
    pub fn new() -> Self {
        // The UI should already include a root
        Ui {
            elements: vec!(Element::new(Style::new())),
            child_connections: vec!(Vec::new()),

            cursor_position: Vector2::new(0.0, 0.0),
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
    }

    pub fn get_mut(&mut self, id: ElementId) -> Option<&mut Element> {
        self.elements.get_mut(id.0)
    }

    pub fn children_of(&self, parent: ElementId) -> &Vec<ElementId> {
        &self.child_connections[parent.0]
    }

    pub fn add_child(&mut self, child: Element, parent: ElementId) -> ElementId {
        // Add the element itself
        // TODO: Allow element removal and re-use element slots
        self.elements.push(child);
        let child_id = ElementId(self.elements.len() - 1);

        // Add the child connections for this element
        self.child_connections.push(Vec::new());
        self.child_connections[parent.0].push(child_id);

        child_id
    }

    pub fn handle_event(&mut self, event: &Input) {
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
            _ => {}
        }
    }

    /// Processes input gathered through handle_event and updates event values on elements.
    pub fn process_input_frame(&mut self) {
        // Go through all elements and see if the mouse is over any of them
        for element in &mut self.elements {
            // Un-set hovering and clicked on this element
            // TODO: Only un-set it on the last frame's element
            element.cursor_state = ElementCursorState::None;
            element.clicked = false;

            // Make sure this element actually captures mouse input
            if element.style.cursor_behavior == CursorBehavior::PassThrough {
                continue;
            }

            // Check if the mouse is over this and if so set it to hovering
            // TODO: Make use of a layering value calculated during calculate_positioning
            if element.positioning.rectangle.contains(self.cursor_position) {
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
            root_id, viewport_size, &mut Vector2::new(0.0, 0.0), &mut 0.0
        );
    }

    pub fn calculate_element_positioning(
        &mut self, element_id: ElementId,
        parent_size: Vector2<f32>, flow_position: &mut Vector2<f32>, flow_margin: &mut f32,
    ) {
        let margined_position;
        let size;

        {
            let element = &mut self[element_id];
            let style = &element.style;

            // Calculate the final position of this element
            let position = match &style.position {
                &Position::Flow => *flow_position,
                // TODO: Make use of parent container position
                &Position::Relative(position) => position,
            };
            margined_position = position + style.margin.max_left(*flow_margin).left_top();

            // Calculate the final size of this element
            size = style.size.to_units(parent_size);

            // If we're positioned using flow, adjust the flow position
            if style.position.is_flow() {
                flow_position.x = margined_position.x + size.x;
                *flow_margin = style.margin.right;
            }

            // Store the calculated data
            element.positioning = Positioning {
                rectangle: Rectangle::start_size(margined_position, size),
            };
        }

        // Now go through all the children as well
        let mut child_flow_position = margined_position;
        for child_id in self.children_of(element_id).clone() {
            self.calculate_element_positioning(
                child_id, size, &mut child_flow_position, &mut 0.0,
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
