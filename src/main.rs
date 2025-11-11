mod dom;

use crate::dom::{create_element, create_text};

fn main() {
    let dom = create_element(
        "div",
        vec![],
        vec![create_element(
            "p",
            vec![("class", "text")],
            vec![create_text("Hello, world!")],
        )],
    );

    println!("{:#?}", dom);
}
