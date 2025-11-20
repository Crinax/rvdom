mod diff;
mod dom;

use diff::diff;
use dom::{h, text};

fn main() {
    let dom = h(
        "div",
        &[("style", "color: red;")],
        vec![h("p", &[], vec![text("Hello")])],
    );
    let new_dom = h(
        "div",
        &[("style", "color: blue;")],
        vec![
            h("p", &[], vec![text("Hello, world!")]),
            h("span", &[], vec![text("New element")]),
        ],
    );

    println!("{:#?}", dom);
    println!("{:#?}", new_dom);
    println!("Patch: {:#?}", diff(&dom, &new_dom));
}
