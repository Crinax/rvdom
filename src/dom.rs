use std::collections::HashMap;

use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub tag_name: String,
    pub props: HashMap<String, String>,
    pub children: Vec<VNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VNode {
    Text(String, String),
    Element(Element, String),
}

impl VNode {
    pub fn element(&self) -> &Element {
        match self {
            VNode::Element(element, _) => element,
            _ => panic!("Not an element node"),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            VNode::Element(_, key) => key,
            VNode::Text(_, key) => key,
        }
    }
}

fn generate_uid(tag_name: Option<&str>) -> String {
    // Generate a unique identifier by randomly sampling 8 characters from
    // the range of 0-9 and a-z.
    let mut rng = rand::rng();

    let chars: Vec<char> = (b'0'..=b'9').chain(b'a'..=b'z').map(Into::into).collect();

    let mut uid: String = (0..8)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect();

    if let Some(tag_name) = tag_name {
        uid.push_str(tag_name);
    }

    uid.insert_str(0, "__");

    uid
}

pub fn h(tag_name: &str, props: &[(&str, &str)], children: Vec<VNode>) -> VNode {
    VNode::Element(
        Element {
            tag_name: tag_name.to_owned(),
            props: props
                .iter()
                .map(|&(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            children,
        },
        generate_uid(Some(tag_name)),
    )
}

pub fn text(text: &str) -> VNode {
    VNode::Text(text.to_owned(), generate_uid(None))
}
