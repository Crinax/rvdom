use std::{collections::HashMap, rc::Rc};

use crate::dom::observer::dom_event::{DomEvent, DomEventType};

pub type Subscriber = fn(Rc<DomEvent>);

#[derive(Debug, Clone, Default)]
pub struct Publisher {
    events: HashMap<DomEventType, Vec<Subscriber>>,
}

impl Publisher {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn subscribe(&mut self, event: DomEventType, subscriber: Subscriber) {
        self.events.entry(event).or_default().push(subscriber);
    }

    pub fn unsubscribe(&mut self, event: &DomEventType, subscriber: Subscriber) {
        if let Some(subscribers) = self.events.get_mut(event) {
            subscribers.retain(|&s| !std::ptr::fn_addr_eq(s, subscriber));
        }
    }

    pub(in crate::dom) fn notify(&self, event: DomEvent) {
        let event = Rc::new(event);
        let event_type = event.clone().into();

        if let Some(subscribers) = self.events.get(&event_type) {
            for &subscriber in subscribers {
                subscriber(event.clone());
            }
        }
    }
}
