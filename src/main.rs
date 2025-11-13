mod dom;

use crate::dom::Document;

fn main() {
    let dom = Document::new(Document::create_element(
        "div",
        vec![],
        vec![Document::create_element(
            "p",
            vec![("class", "text")],
            vec![Document::create_text("Hello, world!")],
        )],
    ));

    println!("{:#?}", dom);
}
