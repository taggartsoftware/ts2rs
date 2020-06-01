// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl BindOptions {
    pub fn new() -> BindOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl RemoteInfo {
    pub fn new() -> RemoteInfo {
        JsCast::unchecked_into(Object::new())
    }
}
impl Socket {
    pub fn new() -> Socket {
        JsCast::unchecked_into(Object::new())
    }
}
impl SocketOptions {
    pub fn new() -> SocketOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<EventEmitter> for Socket {
    fn as_ref(&self) -> &EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<Socket> for EventEmitter {
    fn from(child: Socket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<internal> for Socket {
    fn as_ref(&self) -> &internal {
        JsCast::unchecked_ref(self)
    }
}
impl From<Socket> for internal {
    fn from(child: Socket) -> Self {
        JsCast::unchecked_into(child)
    }
}
