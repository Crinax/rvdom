mod dom;
mod patch;

use dom::{h, text};
use patch::patch;

fn main() {
    let dom = h(
        "div",
        Some("div-123"),
        &[("style", "color: red;")],
        vec![h("p", Some("p-456"), &[], vec![text("Hello")])],
    );
    let new_dom = h(
        "div",
        Some("div-123"),
        &[("style", "color: blue;")],
        vec![
            h("p", Some("p-456"), &[], vec![text("Hello, world!")]),
            h("span", Some("span-789"), &[], vec![text("New element")]),
        ],
    );

    println!("{:#?}", dom);
    println!("{:#?}", new_dom);
    println!("Patch: {:#?}", patch(&dom, &new_dom));
}
