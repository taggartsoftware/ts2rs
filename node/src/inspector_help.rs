// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl InspectorNotification {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Session {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<crate::events::EventEmitter> for Session {
    fn as_ref(&self) -> &crate::events::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<Session> for crate::events::EventEmitter {
    fn from(child: Session) -> Self {
        child.unchecked_into()
    }
}
impl Session {
    pub fn to_events_event_emitter(self) -> crate::events::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_events_event_emitter(&self) -> &crate::events::EventEmitter {
        self.unchecked_ref()
    }
}
