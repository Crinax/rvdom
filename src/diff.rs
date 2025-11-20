use std::collections::HashMap;

use crate::dom::VNode;

#[derive(Debug, Clone, PartialEq)]
pub enum AttributePatch {
    Insert(String, String),
    Update(String, String),
    Remove(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChildPatch {
    Insert(Option<String>, VNode),
    Update(Patch),
    Remove(String),
}

// TODO: Implement patch by index for elements without keys

#[derive(Debug, Clone, PartialEq)]
pub enum Patch {
    None,
    Replace(VNode),
    Update {
        props: Vec<AttributePatch>,
        children: Vec<ChildPatch>,
    },
}

fn props_diff(old: &HashMap<String, String>, new: &HashMap<String, String>) -> Vec<AttributePatch> {
    let mut patches = Vec::new();

    for name in old.keys() {
        if !new.contains_key(name) {
            patches.push(AttributePatch::Remove(name.to_owned()));
        }
    }

    for (name, value) in new.to_owned().into_iter() {
        match old.get(&name) {
            Some(old_value) => {
                if *old_value != value {
                    patches.push(AttributePatch::Update(name, value));
                }
            }
            None => patches.push(AttributePatch::Insert(name, value)),
        }
    }

    patches
}

fn children_diff(old: &[VNode], new: &[VNode]) -> Vec<ChildPatch> {
    let mut patches = Vec::new();
    let mut remove_patches = Vec::new();
    let mut insert_patches = Vec::new();
    let mut update_patches = Vec::new();
    let mut old_key_index_map = HashMap::new();
    let mut new_key_index_map: HashMap<&str, usize> = HashMap::new();
    let new_index_key_map: HashMap<usize, &str> = new
        .iter()
        .enumerate()
        .map(|(i, node)| (i, node.key()))
        .collect();

    for (i, node) in old.iter().enumerate() {
        old_key_index_map.insert(node.key(), i);
    }

    for (i, node) in new.iter().enumerate() {
        new_key_index_map.insert(node.key(), i);

        match old_key_index_map.get(node.key()) {
            Some(old_index) => {
                update_patches.push(ChildPatch::Update(diff(&old[*old_index], &node)));
            }
            None => {
                let key = *new_index_key_map.get(&i).unwrap();

                insert_patches.push(ChildPatch::Insert(Some(key.to_owned()), node.to_owned()));
            }
        }
    }

    for node in old {
        let key = node.key();

        if !new_key_index_map.contains_key(key) {
            remove_patches.push(ChildPatch::Remove(key.to_owned()));
        }
    }

    patches.extend(remove_patches);
    patches.extend(insert_patches);
    patches.extend(update_patches);

    patches
}

pub fn diff(old: &VNode, new: &VNode) -> Patch {
    if old == new {
        return Patch::None;
    }

    match (old, new) {
        (VNode::Text(_, _), el) => Patch::Replace(el.to_owned()),
        (_, VNode::Text(text, key)) => Patch::Replace(VNode::Text(text.to_owned(), key.to_owned())),

        (old, new) => {
            if old.key() != new.key() {
                Patch::Replace(new.to_owned())
            } else {
                let old = old.element();
                let new = new.element();

                let props: Vec<AttributePatch> = props_diff(&old.props, &new.props);
                let children: Vec<ChildPatch> = children_diff(&old.children, &new.children);

                if props.is_empty() && children.is_empty() {
                    Patch::None
                } else {
                    Patch::Update { props, children }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::{h, text};

    use super::*;

    #[test]
    fn test_diff_with_same_text_nodes() {
        let old = text("Hello, world!");
        let new = text("Hello, world!");

        assert_eq!(diff(&old, &new), Patch::None);
    }

    #[test]
    fn test_diff_with_different_text_nodes() {
        let old = text("Hello!");
        let new = text("Hello, world!");

        assert_eq!(diff(&old, &new), Patch::Replace(new.clone()));
    }

    #[test]
    fn test_diff_with_same_element_nodes() {
        let old = h("div", &[], vec![]);
        let new = h("div", &[], vec![]);

        assert_eq!(diff(&old, &new), Patch::None);
    }

    #[test]
    fn test_diff_with_different_element_nodes() {
        let old = h("div", &[], vec![]);
        let new = h("span", &[], vec![]);

        assert_eq!(diff(&old, &new), Patch::Replace(new.clone()));
    }

    #[test]
    fn test_diff_with_same_element_nodes_with_same_props() {
        let old = h("div", &[("style", "color: red;")], vec![]);
        let new = h("div", &[("style", "color: red;")], vec![]);

        assert_eq!(diff(&old, &new), Patch::None);
    }

    #[test]
    fn test_diff_with_same_element_nodes_with_different_props() {
        let old = h("div", &[("style", "color: red;")], vec![]);
        let new = h("div", &[("style", "color: blue;")], vec![]);

        assert_eq!(
            diff(&old, &new),
            Patch::Update {
                props: vec![AttributePatch::Update(
                    "style".to_owned(),
                    "color: blue;".to_owned()
                )],
                children: vec![],
            }
        );
    }

    #[test]
    fn test_diff_with_same_element_nodes_with_different_text_children() {
        let old = h("div", &[], vec![text("Hello!")]);
        let new = h("div", &[], vec![text("Hello, world!")]);

        assert_eq!(diff(&old, &new), Patch::Replace(new.clone()));
    }

    #[test]
    fn test_diff_with_same_element_nodes_with_same_text_children() {
        let old = h("div", &[], vec![text("Hello!")]);
        let new = h("div", &[], vec![text("Hello!")]);

        assert_eq!(diff(&old, &new), Patch::None);
    }

    #[test]
    fn test_diff_with_same_element_nodes_with_different_children() {
        let old = h("div", &[], vec![h("p", &[], vec![])]);
        let new = h("div", &[], vec![h("span", &[], vec![])]);

        assert_eq!(
            diff(&old, &new),
            Patch::Update {
                props: vec![],
                children: vec![
                    ChildPatch::Remove("p-456".to_owned()),
                    ChildPatch::Insert(None, h("span", &[], vec![]))
                ]
            }
        );
    }

    #[test]
    fn test_diff_with_same_element_nodes_with_same_children_with_different_props() {
        let old = h(
            "div",
            &[],
            vec![h("span", &[("style", "color: blue;")], vec![])],
        );
        let new = h(
            "div",
            &[],
            vec![h("span", &[("style", "color: red;")], vec![])],
        );

        assert_eq!(
            diff(&old, &new),
            Patch::Update {
                props: vec![],
                children: vec![ChildPatch::Update(Patch::Update {
                    props: vec![AttributePatch::Update(
                        "style".to_owned(),
                        "color: red;".to_owned()
                    )],
                    children: vec![]
                })],
            }
        );
    }

    #[test]
    fn test_same_nodes_with_different_keys() {
        let old = h("div", &[], vec![]);
        let new = h("div", &[], vec![]);

        assert_eq!(diff(&old, &new), Patch::Replace(new.clone()));
    }
}
