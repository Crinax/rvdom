use crate::dom::Element;

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}
