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

            for name in old_element.props.keys() {
                if !new_element.props.contains_key(name) {
                    props.push(AttributePatch::Remove(name.to_owned()));
                }
            }

            for (name, value) in new_element.props.to_owned().into_iter() {
                match old_element.props.get(&name) {
                    Some(old_value) => {
                        if *old_value != value {
                            props.push(AttributePatch::Update(name, value));
                        }
                    }
                    None => props.push(AttributePatch::Insert(name, value)),
                }
            }

            let same_elements_patches: Vec<_> = new_element
                .children
                .to_owned()
                .into_iter()
                .flat_map(|child| {
                    old_element
                        .children
                        .to_owned()
                        .into_iter()
                        .filter_map(move |old_child| match (&old_child, &child) {
                            (
                                VNode::Element(Element { key: old_key, .. }),
                                VNode::Element(Element { key: new_key, .. }),
                            ) => {
                                if old_key.as_deref().is_some_and(|key| {
                                    new_key.as_deref().is_some_and(|new_key| key == new_key)
                                }) {
                                    Some(ChildPatch::Update(patch(&old_child, &child)))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|patch| patch != &ChildPatch::Update(Patch::None))
                .collect();

            children.extend(same_elements_patches);

            Patch::Update { props, children }
        }

        _ => Patch::None,
    }
}
