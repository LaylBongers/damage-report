use std::ops::{Index, IndexMut};
use cgmath::{Vector2};
use style::{Style, Size, Position};
use {Element, Positioning};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(usize);

pub struct Ui {
    elements: Vec<Element>,
    child_connections: Vec<Vec<ElementId>>
}

impl Ui {
    pub fn new() -> Self {
        // The UI should already include a root
        Ui {
            elements: vec!(Element::new(Style::new())),
            child_connections: vec!(Vec::new()),
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
                position: margined_position,
                size: size,
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
