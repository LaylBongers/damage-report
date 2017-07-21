use std::ops::{Index, IndexMut};
use style::{Style};
use {Element};

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
