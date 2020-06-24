// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl DOMEventTarget {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl NodeEventTarget {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl internal {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl EventEmitter {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<node_js::EventEmitter> for internal {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<internal> for node_js::EventEmitter {
    fn from(child: internal) -> Self {
        child.unchecked_into()
    }
}
impl internal {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::EventEmitter> for EventEmitter {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<EventEmitter> for node_js::EventEmitter {
    fn from(child: EventEmitter) -> Self {
        child.unchecked_into()
    }
}
impl EventEmitter {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
