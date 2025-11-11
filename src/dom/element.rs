use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::dom::{
    Node,
    observer::{DomEvent, Publisher},
};

#[derive(Debug, Clone, Default)]
pub struct Element {
    pub tag_name: String,
    pub events: Publisher,
    attributes: HashMap<String, String>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Element {
    pub fn new(tag_name: &str) -> Self {
        Element {
            tag_name: tag_name.to_string(),
            ..Default::default()
        }
    }

    pub fn append_child(&mut self, child: Node) {
        let child = Rc::new(RefCell::new(child));

        self.children.push(child.clone());
        self.events.notify(DomEvent::NodeAdded(&child.borrow()));
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        let result = self.attributes.insert(key.to_string(), value.to_string());

        self.events.notify(DomEvent::AttributeChanged(
            self,
            key.to_string(),
            (value.to_string(), result),
        ));
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn attributes(&self) -> impl IntoIterator<Item = (&String, &String)> {
        self.attributes.iter()
    }

    pub fn children(&self) -> impl IntoIterator<Item = &Rc<RefCell<Node>>> {
        self.children.iter()
    }

    pub fn children_count(&self) -> usize {
        self.children.len()
    }
}
