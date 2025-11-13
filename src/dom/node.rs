use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum VNode {
    Text {
        value: String,
        key: Option<String>,
    },
    Element {
        key: Option<String>,
        tag_name: String,
        props: HashMap<String, String>,
        children: Vec<Rc<RefCell<VNode>>>,
    },
}

impl VNode {
    pub fn element(tag_name: &str, key: Option<&str>) -> Self {
        VNode::Element {
            key: key.map(|k| k.to_owned()),
            tag_name: tag_name.to_owned(),
            props: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn text(value: &str, key: Option<&str>) -> Self {
        VNode::Text {
            value: value.to_owned(),
            key: key.map(|k| k.to_owned()),
        }
    }

    pub fn key(&self) -> Option<&str> {
        match self {
            VNode::Text { key, .. } => key.as_deref(),
            VNode::Element { key, .. } => key.as_deref(),
        }
    }

    pub fn append_child(&mut self, child: VNode) {
        match self {
            VNode::Element { children, .. } => children.push(Rc::new(RefCell::new(child))),
            _ => (),
        }
    }

    pub fn set_property(&mut self, name: &str, value: &str) {
        match self {
            VNode::Element { props, .. } => {
                props.insert(name.to_string(), value.to_string());
            }
            _ => (),
        }
    }

    pub fn get_properties(&self, name: &str) -> Option<&String> {
        match self {
            VNode::Element { props, .. } => props.get(name),
            _ => None,
        }
    }

    pub fn children(&self) -> impl IntoIterator<Item = &Rc<RefCell<VNode>>> {
        match self {
            VNode::Element { children, .. } => children.iter(),
            _ => [].iter(),
        }
    }

    pub fn children_count(&self) -> usize {
        match self {
            VNode::Element { children, .. } => children.len(),
            _ => 0,
        }
    }
}
