// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl DefaultDeserializer {
    pub fn new() -> DefaultDeserializer {
        JsCast::unchecked_into(Object::new())
    }
}
impl DefaultSerializer {
    pub fn new() -> DefaultSerializer {
        JsCast::unchecked_into(Object::new())
    }
}
impl Deserializer {
    pub fn new() -> Deserializer {
        JsCast::unchecked_into(Object::new())
    }
}
impl HeapCodeStatistics {
    pub fn new() -> HeapCodeStatistics {
        JsCast::unchecked_into(Object::new())
    }
}
impl HeapInfo {
    pub fn new() -> HeapInfo {
        JsCast::unchecked_into(Object::new())
    }
}
impl HeapSpaceInfo {
    pub fn new() -> HeapSpaceInfo {
        JsCast::unchecked_into(Object::new())
    }
}
impl Serializer {
    pub fn new() -> Serializer {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<crate::v8::Deserializer> for crate::v8::DefaultDeserializer {
    fn as_ref(&self) -> &crate::v8::Deserializer {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::v8::DefaultDeserializer> for crate::v8::Deserializer {
    fn from(child: crate::v8::DefaultDeserializer) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<crate::v8::Serializer> for crate::v8::DefaultSerializer {
    fn as_ref(&self) -> &crate::v8::Serializer {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::v8::DefaultSerializer> for crate::v8::Serializer {
    fn from(child: crate::v8::DefaultSerializer) -> Self {
        JsCast::unchecked_into(child)
    }
}
