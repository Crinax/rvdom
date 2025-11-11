mod dom;

use crate::dom::{Document, DomEvent, DomEventType, Node};

fn main() {
    let mut root = Document::create_element("div", vec![], vec![]);

    if let Node::Element(ref mut elem) = root {
        elem.events.subscribe(DomEventType::NodeAdded, |event| {
            if let DomEvent::NodeAdded(node) = &*event {
                println!("Node added: {:#?}", node);
            }
        });

        elem.events
            .subscribe(DomEventType::AttributeChanged, |event| {
                if let DomEvent::AttributeChanged(element, key, (new_value, old_value)) = &*event {
                    println!(
                        "Attribute changed on element {:?}: {} from {:?} to {:?}",
                        element.tag_name, key, old_value, new_value
                    );
                }
            });

        elem.append_child(Document::create_element(
            "p",
            vec![("class", "text")],
            vec![Document::create_text("Hello, world!")],
        ));
        elem.set_attribute("class", "container");
    }

    let dom = Document::new(root);

    println!("{:#?}", dom);
}
