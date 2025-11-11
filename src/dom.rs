mod element;
mod node;

pub use element::Element;
pub use node::Node;

#[derive(Debug, Clone)]
pub struct Document {
    root: Node,
}

impl Document {
    pub fn new(root: Node) -> Self {
        Document { root }
    }

    pub fn create_element(
        tag_name: &str,
        attributes: Vec<(&str, &str)>,
        children: Vec<Node>,
    ) -> Node {
        let mut element = Element::new(tag_name);

        for (key, value) in attributes {
            element.set_attribute(key, value);
        }

        for child in children {
            element.append_child(child);
        }

        Node::Element(element)
    }

    pub fn create_text(text: &str) -> Node {
        Node::Text(text.to_string())
    }
}
