use crate::dom::VNode;

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

pub fn patch(old: VNode, new: VNode) -> Patch {
    if old == new {
        return Patch::None;
    }

    match (old, new) {
        (VNode::Text(_), node) => Patch::Replace(node),
        (_, VNode::Text(text)) => Patch::Replace(VNode::Text(text)),

        (VNode::Element { tag_name: old_tag_name, key: _, props: old_props, children: old_children }, VNode::Element { tag_name: new_tag_name, key: _, props: new_props, children: new_children }) => {
            if old_tag_name != new_tag_name {
                return Patch::Replace(new);
            }

            let mut props = Vec::new();
            let mut children = Vec::new();

            for (k, v) in new_props {
                if old_props.contains_key(&k) {
                    if old_props[&k] != v {
                        props.push(AttributePatch::Update(k, v));
                    }
                } else {
                    props.push(AttributePatch::Insert(k, v));
                }
            }

            for k in old_props.keys() {
                if !new_props.contains_key(k) {
                    props.push(AttributePatch::Remove(k.to_owned()));
                }
            }

            for (i, (old_child, new_child)) in old_children.iter().zip(new_children.iter()).enumerate() {
                if old_child != new_child {
                    children.push(ChildPatch::Update(patch(old_child.clone(), new_child.clone())));
                }
            }

            for i in old_children.len()..new_children.len() {
                children.push(ChildPatch::Insert(new_children[i].clone()));
            }

            for i in new_children.len()..old_children.len() {
                children.push(ChildPatch::Remove(old_children[i].clone()));
            }

            Patch::Update { props, children }
        }


        _ => Patch::None,
    }
}
