mod node;

pub use node::VNode;

#[derive(Debug, Clone)]
pub struct Document {
    pub root: VNode,
}

impl Document {
    pub fn new(root: VNode) -> Self {
        Document { root }
    }

    pub fn create_element(
        tag_name: &str,
        attributes: Vec<(&str, &str)>,
        children: Vec<VNode>,
    ) -> VNode {
        let mut element = VNode::element(tag_name, None);

        for (key, value) in attributes {
            element.set_property(key, value);
        }

        for child in children {
            element.append_child(child);
        }

        element
    }

    pub fn create_text(text: &str) -> VNode {
        VNode::text(text, None)
    }
}
