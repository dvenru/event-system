use std::collections::HashMap;

pub struct EventBus<T: Clone> {
    listeners: HashMap<String, Vec<Box<dyn Fn(T)>>>,
}

impl<T: Clone> EventBus<T> {
    pub fn new() -> Self {
        EventBus {
            listeners: HashMap::new(),
        }
    }

    pub fn subscribe<F: 'static + Fn(T)>(&mut self, event_type: &str, listener: F) {
        self.listeners.entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }

    pub fn publish(&self, event_type: &str, data: T) {
        if let Some(listeners) = self.listeners.get(event_type) {
            for listener in listeners {
                listener(data.clone());
            }
        }
    }
}
