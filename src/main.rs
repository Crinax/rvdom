mod dom;

use dom::{h, text};

fn main() {
    let dom = h(
        "div",
        None,
        &[("style", "color: red;")],
        vec![h("p", None, &[], vec![text("Hello")])],
    );

    println!("{:#?}", dom);
}
