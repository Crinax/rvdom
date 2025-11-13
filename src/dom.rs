use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VNode {
    Text(String),
    Element {
        tag_name: String,
        key: Option<String>,
        props: HashMap<String, String>,
        children: Vec<VNode>,
    },
}

pub fn h(tag_name: &str, key: Option<&str>, props: &[(&str, &str)], children: Vec<VNode>) -> VNode {
    VNode::Element {
        tag_name: tag_name.to_owned(),
        key: key.map(|s| s.to_owned()),
        props: props
            .iter()
            .map(|&(k, v)| (k.to_owned(), v.to_owned()))
            .collect(),
        children,
    }
}

pub fn text(text: &str) -> VNode {
    VNode::Text(text.to_owned())
}
