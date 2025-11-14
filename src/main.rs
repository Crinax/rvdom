mod dom;
mod patch;

use dom::{h, text};

fn main() {
    let dom = text("Hello");
    let new_dom = h(
        "div",
        None,
        &[("style", "color: red;")],
        vec![h("p", None, &[], vec![text("Hello")])],
    );

    println!("{:#?}", dom);
    println!("{:#?}", new_dom);
    println!("Patch: {:#?}", patch::patch(dom, new_dom));
}
