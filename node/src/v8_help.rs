// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl DefaultDeserializer {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl DefaultSerializer {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Deserializer {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl HeapCodeStatistics {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl HeapInfo {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl HeapSpaceInfo {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Serializer {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<Deserializer> for DefaultDeserializer {
    fn as_ref(&self) -> &Deserializer {
        self.unchecked_ref()
    }
}
impl From<DefaultDeserializer> for Deserializer {
    fn from(child: DefaultDeserializer) -> Self {
        child.unchecked_into()
    }
}
impl DefaultDeserializer {
    pub fn to_deserializer(self) -> Deserializer {
        self.unchecked_into()
    }
    pub fn as_deserializer(&self) -> &Deserializer {
        self.unchecked_ref()
    }
}
impl AsRef<Serializer> for DefaultSerializer {
    fn as_ref(&self) -> &Serializer {
        self.unchecked_ref()
    }
}
impl From<DefaultSerializer> for Serializer {
    fn from(child: DefaultSerializer) -> Self {
        child.unchecked_into()
    }
}
impl DefaultSerializer {
    pub fn to_serializer(self) -> Serializer {
        self.unchecked_into()
    }
    pub fn as_serializer(&self) -> &Serializer {
        self.unchecked_ref()
    }
}
