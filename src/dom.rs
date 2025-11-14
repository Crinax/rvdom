use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub tag_name: String,
    pub key: Option<String>,
    pub props: HashMap<String, String>,
    pub children: Vec<VNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VNode {
    Text(String),
    Element(Element),
}

impl VNode {
    pub fn is_text(&self) -> bool {
        match self {
            VNode::Text(_) => true,
            _ => false,
        }
    }

    pub fn is_element(&self) -> bool {
        match self {
            VNode::Element(_) => true,
            _ => false,
        }
    }

    pub fn text(&self) -> &String {
        match self {
            VNode::Text(text) => text,
            _ => panic!("Not a text node"),
        }
    }

    pub fn element(&self) -> &Element {
        match self {
            VNode::Element(element) => element,
            _ => panic!("Not an element node"),
        }
    }
}

pub fn h(tag_name: &str, key: Option<&str>, props: &[(&str, &str)], children: Vec<VNode>) -> VNode {
    VNode::Element(Element {
        tag_name: tag_name.to_owned(),
        key: key.map(|s| s.to_owned()),
        props: props
            .iter()
            .map(|&(k, v)| (k.to_owned(), v.to_owned()))
            .collect(),
        children,
    })
}

pub fn text(text: &str) -> VNode {
    VNode::Text(text.to_owned())
}
