use std::ops::{Index, IndexMut};

use style::{Style};
use {Element};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(pub usize);

pub struct Elements {
    elements: Vec<Option<Element>>,
    child_connections: Vec<(ElementId, Vec<ElementId>)>, // Parent, then children
    next_inner_id: i32,
}

impl Elements {
    pub fn new() -> Self {
        let mut root = Element::new(Style::new());
        root.inner_id = 0;

        // The UI should already include a root
        Elements {
            elements: vec!(Some(root)),
            child_connections: vec!((ElementId(0), Vec::new())),
            next_inner_id: 1,
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

    pub fn parent_of(&self, child: ElementId) -> ElementId {
        self.child_connections[child.0].0
    }

    pub fn children_of(&self, parent: ElementId) -> &Vec<ElementId> {
        &self.child_connections[parent.0].1
    }

    pub fn all(&self) -> &Vec<Option<Element>> {
        &self.elements
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
}

impl Index<ElementId> for Elements {
    type Output = Element;

    fn index<'a>(&'a self, index: ElementId) -> &'a Element {
        self.get(index).expect("Unable to find element")
    }
}

impl IndexMut<ElementId> for Elements {
    fn index_mut<'a>(&'a mut self, index: ElementId) -> &'a mut Element {
        self.get_mut(index).expect("Unable to find element")
    }
}
