use screenmath::{Rectangle, Lrtb};
use cgmath::{Vector2, Point2};
use input::{Input, Motion, Button, MouseButton, Key, ButtonState, ButtonArgs};
use rusttype::{Font};

use style::{Size, FlowDirection};
use {ElementCursorState, ElementBehavior, Elements, ElementId};

/// Represents a font by index in the font list.
// TODO: Make this more of an opaque type, it should just be an arbitrary font reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(pub usize);

/// The base UI type. Contains the elements, fonts, and the current input state of the UI.
pub struct Ui {
    pub elements: Elements,
    pub fonts: Vec<Font<'static>>,

    cursor_position: Point2<f32>,
    cursor_active_element: Option<ElementId>,
    text_active_element: Option<ElementId>,

    // pressed/released are reset every frame, state is persistent
    cursor_pressed: bool,
    cursor_released: bool,
    cursor_state: bool,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            elements: Elements::new(),
            fonts: Vec::new(),

            cursor_position: Point2::new(0.0, 0.0),
            cursor_active_element: None,
            text_active_element: None,

            cursor_pressed: false,
            cursor_released: false,
            cursor_state: false,
        }
    }

    pub fn cursor_active_element(&self) -> Option<ElementId> {
        self.cursor_active_element
    }

    pub fn handle_event(&mut self, event: &Input) {
        // In case we have text input
        let elements = &mut self.elements;
        let el_text = self.text_active_element
            .and_then(|id| elements.get_mut(id))
            .and_then(|element| element.text.as_mut());

        match *event {
            Input::Button(ButtonArgs {state, button, scancode: _scancode}) => {
                match button {
                    Button::Mouse(MouseButton::Left) => {
                        if state == ButtonState::Press {
                            self.cursor_pressed = true;
                            self.cursor_released = false;
                            self.cursor_state = true;
                        } else {
                            self.cursor_pressed = false;
                            self.cursor_released = true;
                            self.cursor_state = false;
                        }
                    },
                    Button::Keyboard(Key::Backspace) => {
                        if state == ButtonState::Press {
                            // We received a backspace, remove text from the element
                            if let Some(el_text) = el_text {
                                el_text.text_mut().pop();
                            }
                        }
                    },
                    _ => (),
                }
            },
            Input::Move(Motion::MouseCursor(x, y)) =>
                self.cursor_position = Point2::new(x, y).cast(),
            Input::Text(ref text) => {
                // We received text, so pass it to the element
                if let Some(el_text) = el_text {
                    el_text.text_mut().push_str(text);
                }
            },
            _ => {}
        }
    }

    /// Processes input gathered through handle_event and updates event values on elements.
    pub fn process_input_frame(&mut self) {
        // Reset the previous input frame
        if let Some(id) = self.cursor_active_element.take() {
            if let Some(ref mut element) = self.elements.get_mut(id) {
                element.cursor_state = ElementCursorState::None;
                element.clicked = false;
            }
        }

        // If we clicked, clear the previous active text field element, so when we click outside
        // with one focused, it gets un-focused
        if self.cursor_released {
            if let Some(id) = self.text_active_element.take() {
                if let Some(ref mut element) = self.elements.get_mut(id) {
                    element.focused = false;
                }
            }
        }

        // Go through all elements and see if the mouse is over any of them
        {
            let all_elements = self.elements.all();
            for id in 0..all_elements.len() {
                if let Some(ref element) = all_elements[id] {
                    // Make sure this element actually captures mouse input
                    if element.behavior == ElementBehavior::Passive {
                        continue;
                    }

                    // Check if the mouse is over this and if so set it to hovering
                    // TODO: Make use of a layering value calculated during calculate_positioning
                    if element.positioning.container.contains(self.cursor_position) {
                        // Remember this element so we can update it later if it's indeed on top
                        self.cursor_active_element = Some(ElementId(id));
                    }
                }
            }
        }

        // If anything became active again, mark it as such
        if let Some(id) = self.cursor_active_element {
            let element = self.elements.get_mut(id).unwrap();
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
                if element.behavior == ElementBehavior::TextField {
                    element.focused = true;
                    self.text_active_element = Some(id);
                }
            }
        }

        // Reset cursor pressed data
        self.cursor_pressed = false;
        self.cursor_released = false;
    }

    pub fn update_layout(&mut self, viewport_size: Vector2<f32>) {
        let root_id = self.elements.root_id();

        // Lock the root to match the viewport
        {
            let style = &mut self.elements[root_id].style;
            style.size = Size::units(viewport_size.x, viewport_size.y);
        }

        // Start off the calculation at the root
        self.update_element_layout(
            root_id,
            &Rectangle::start_size(Point2::new(0.0, 0.0), viewport_size), &Lrtb::uniform(0.0),
            &mut Point2::new(0.0, 0.0), &mut 0.0, FlowDirection::Right,
        );
    }

    fn update_element_layout(
        &mut self, element_id: ElementId,
        parent_container: &Rectangle<f32>, parent_padding: &Lrtb,
        flow_cursor: &mut Point2<f32>, flow_margin: &mut f32, flow_direction: FlowDirection,
    ) {
        let our_container;
        let our_padding;
        let mut child_flow_cursor;
        let mut child_flow_margin;
        let child_flow_direction;

        {
            let element = self.elements.get_mut(element_id).unwrap();
            element.update_layout(
                parent_container, parent_padding,
                flow_cursor, flow_margin, flow_direction,
                &self.fonts,
            );

            // Calculate the flow data needed by the children based on this element's flow data
            our_container = element.positioning.container.clone();
            our_padding = element.style.padding.clone();
            child_flow_direction = element.style.flow_direction;
            child_flow_margin = element.style.padding.left;
            child_flow_cursor = child_flow_direction.flow_start(&our_container);
        }

        // Now go through all the children as well
        for child_id in self.elements.children_of(element_id).clone() {
            self.update_element_layout(
                child_id, &our_container, &our_padding,
                &mut child_flow_cursor, &mut child_flow_margin, child_flow_direction,
            );
        }
    }
}
