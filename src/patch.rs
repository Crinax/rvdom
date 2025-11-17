use std::collections::HashMap;

use crate::dom::{Element, VNode};

#[derive(Debug, Clone, PartialEq)]
pub enum AttributePatch {
    Insert(String, String),
    Update(String, String),
    Remove(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChildPatch {
    Insert(VNode),
    Update(Patch),
    Remove(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Patch {
    None,
    Replace(VNode),
    Update {
        props: Vec<AttributePatch>,
        children: Vec<ChildPatch>,
    },
}

fn are_text_nodes_same(old: &[VNode], new: &[VNode]) -> bool {
    let mut old_text_nodes_iter = old.iter().filter(|child| child.is_text());
    let mut new_text_nodes_iter = new.iter().filter(|child| child.is_text());

    let has_same_size = old_text_nodes_iter.clone().count() == new_text_nodes_iter.clone().count();

    let all_old_text_nodes_are_in_new = old_text_nodes_iter.all(|child| new.contains(child));
    let all_new_text_nodes_are_in_old = new_text_nodes_iter.all(|child| old.contains(child));

    has_same_size && all_old_text_nodes_are_in_new && all_new_text_nodes_are_in_old
}

fn diff_props(
    old_props: &HashMap<String, String>,
    new_props: &HashMap<String, String>,
) -> Vec<AttributePatch> {
    let mut patches = Vec::new();

    for name in old_props.keys() {
        if !new_props.contains_key(name) {
            patches.push(AttributePatch::Remove(name.to_owned()));
        }
    }

    for (name, value) in new_props.to_owned().into_iter() {
        match old_props.get(&name) {
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

fn diff_children(old: &[VNode], new: &[VNode]) -> Vec<ChildPatch> {
    let mut patches = Vec::new();

    for old_child in old {
        match old_child {
            VNode::Element(Element {
                key: Some(old_key), ..
            }) => {
                if !new.iter().any(|new_child| match new_child {
                    VNode::Element(Element {
                        key: Some(new_key), ..
                    }) => old_key == new_key,
                    _ => false,
                }) {
                    patches.push(ChildPatch::Remove(old_key.to_owned()));
                }
            }
            _ => {}
        }
    }

    for new_child in new {
        match new_child {
            VNode::Element(Element {
                key: Some(new_key), ..
            }) => {
                if !old.iter().any(|old_child| match old_child {
                    VNode::Element(Element {
                        key: Some(old_key), ..
                    }) => old_key == new_key,
                    _ => false,
                }) {
                    patches.push(ChildPatch::Insert(new_child.to_owned()));
                }
            }
            _ => {}
        }
    }

    for old_child in old {
        match old_child {
            VNode::Element(Element {
                key: Some(old_key), ..
            }) => {
                if let Some(new_child) = new.iter().find(|new_child| match new_child {
                    VNode::Element(Element {
                        key: Some(new_key), ..
                    }) => old_key == new_key,
                    _ => false,
                }) {
                    let new_patch = patch(old_child, new_child);
                    if new_patch != Patch::None {
                        patches.push(ChildPatch::Update(new_patch));
                    }
                }
            }
            _ => {}
        }
    }

    patches
}

pub fn patch(old: &VNode, new: &VNode) -> Patch {
    if old == new {
        return Patch::None;
    }

    match (old, new) {
        (VNode::Text(_), node) => Patch::Replace(node.to_owned()),
        (_, VNode::Text(text)) => Patch::Replace(VNode::Text(text.to_owned())),

        (old_element, new_element) => {
            let old_element = old_element.element();
            let new_element = new_element.element();

            let mut props = Vec::new();
            let mut children = Vec::new();

            if !are_text_nodes_same(&old_element.children, &new_element.children) {
                return Patch::Replace(VNode::Element(new_element.to_owned()));
            }

            props.extend(diff_props(&old_element.props, &new_element.props));

            children.extend(diff_children(&old_element.children, &new_element.children));

            Patch::Update { props, children }
        }
    }
}
