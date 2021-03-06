// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl MessageChannel {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl MessagePort {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Worker {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl WorkerOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<crate::events::EventEmitter> for MessagePort {
    fn as_ref(&self) -> &crate::events::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<MessagePort> for crate::events::EventEmitter {
    fn from(child: MessagePort) -> Self {
        child.unchecked_into()
    }
}
impl MessagePort {
    pub fn to_events_event_emitter(self) -> crate::events::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_events_event_emitter(&self) -> &crate::events::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<crate::events::EventEmitter> for Worker {
    fn as_ref(&self) -> &crate::events::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<Worker> for crate::events::EventEmitter {
    fn from(child: Worker) -> Self {
        child.unchecked_into()
    }
}
impl Worker {
    pub fn to_events_event_emitter(self) -> crate::events::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_events_event_emitter(&self) -> &crate::events::EventEmitter {
        self.unchecked_ref()
    }
}
