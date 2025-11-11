use std::rc::Rc;

use crate::dom::{Element, Node};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomEventType {
    NodeAdded,
    AttributeChanged,
}

#[derive(Debug, Clone)]
pub enum DomEvent<'a> {
    NodeAdded(&'a Node),
    AttributeChanged(&'a Element, String, (String, Option<String>)),
}

impl<'a> From<&DomEvent<'a>> for DomEventType {
    fn from(event: &DomEvent) -> Self {
        match event {
            DomEvent::NodeAdded(_) => DomEventType::NodeAdded,
            DomEvent::AttributeChanged(_, _, _) => DomEventType::AttributeChanged,
        }
    }
}

impl<'a> From<Rc<DomEvent<'a>>> for DomEventType {
    fn from(event: Rc<DomEvent>) -> Self {
        (&*event).into()
    }
}
