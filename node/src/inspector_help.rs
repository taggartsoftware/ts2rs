// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl InspectorNotification {
    pub fn new() -> InspectorNotification {
        JsCast::unchecked_into(Object::new())
    }
}
impl Session {
    pub fn new() -> Session {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<EventEmitter> for crate::inspector::Session {
    fn as_ref(&self) -> &EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::inspector::Session> for EventEmitter {
    fn from(child: crate::inspector::Session) -> Self {
        JsCast::unchecked_into(child)
    }
}
