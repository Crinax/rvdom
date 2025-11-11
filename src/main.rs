mod dom;

use crate::dom::create_element;

fn main() {
    let dom = create_element(
        "div",
        vec![],
        vec![create_element("p", vec![("class", "text")], vec![])],
    );

    println!("{:#?}", dom);
}
