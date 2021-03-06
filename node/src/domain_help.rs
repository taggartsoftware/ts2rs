// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl Domain {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<node_js::Domain> for Domain {
    fn as_ref(&self) -> &node_js::Domain {
        self.unchecked_ref()
    }
}
impl From<Domain> for node_js::Domain {
    fn from(child: Domain) -> Self {
        child.unchecked_into()
    }
}
impl Domain {
    pub fn to_node_js_domain(self) -> node_js::Domain {
        self.unchecked_into()
    }
    pub fn as_node_js_domain(&self) -> &node_js::Domain {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::EventEmitter> for Domain {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<Domain> for node_js::EventEmitter {
    fn from(child: Domain) -> Self {
        child.unchecked_into()
    }
}
impl Domain {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
