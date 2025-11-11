use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Default)]
pub struct Element {
    pub tag_name: String,
    attributes: HashMap<String, String>,
    children: Vec<Rc<RefCell<Element>>>,
}

impl Element {
    pub fn new(tag_name: &str) -> Self {
        Element {
            tag_name: tag_name.to_string(),
            ..Default::default()
        }
    }

    pub fn append_child(&mut self, child: Element) {
        self.children.push(Rc::new(RefCell::new(child)));
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn attributes(&self) -> impl IntoIterator<Item = (&String, &String)> {
        self.attributes.iter()
    }

    pub fn attributes_mut(&mut self) -> impl IntoIterator<Item = (&String, &mut String)> {
        self.attributes.iter_mut()
    }

    pub fn children(&self) -> impl IntoIterator<Item = &Rc<RefCell<Element>>> {
        self.children.iter()
    }

    pub fn children_mut(&mut self) -> impl IntoIterator<Item = &mut Rc<RefCell<Element>>> {
        self.children.iter_mut()
    }

    pub fn children_count(&self) -> usize {
        self.children.len()
    }
}
