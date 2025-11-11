mod element;

pub use element::Element;

pub fn create_element(
    tag_name: &str,
    attributes: Vec<(&str, &str)>,
    children: Vec<Element>,
) -> Element {
    let mut element = Element::new(tag_name);

    for (key, value) in attributes {
        element.set_attribute(key, value);
    }

    for child in children {
        element.append_child(child);
    }

    element
}