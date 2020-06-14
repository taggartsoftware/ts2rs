// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl Interface {
    pub fn new() -> Interface {
        JsCast::unchecked_into(Object::new())
    }
}
impl Key {
    pub fn new() -> Key {
        JsCast::unchecked_into(Object::new())
    }
}
impl ReadLineOptions {
    pub fn new() -> ReadLineOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<node_js::EventEmitter> for Interface {
    fn as_ref(&self) -> &node_js::EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<Interface> for node_js::EventEmitter {
    fn from(child: Interface) -> Self {
        JsCast::unchecked_into(child)
    }
}
