// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl Buffer {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Console {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Error {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ErrorConstructor {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ImportMeta {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl NodeExtensions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl NodeModule {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl NodeRequire {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl NodeRequireCache {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl NodeRequireFunction {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl RequireResolve {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SymbolConstructor {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<Uint8Array> for Buffer {
    fn as_ref(&self) -> &Uint8Array {
        self.unchecked_ref()
    }
}
impl From<Buffer> for Uint8Array {
    fn from(child: Buffer) -> Self {
        child.unchecked_into()
    }
}
impl Buffer {
    pub fn to_uint8_array(self) -> Uint8Array {
        self.unchecked_into()
    }
    pub fn as_uint8_array(&self) -> &Uint8Array {
        self.unchecked_ref()
    }
}
impl AsRef<NodeRequireFunction> for NodeRequire {
    fn as_ref(&self) -> &NodeRequireFunction {
        self.unchecked_ref()
    }
}
impl From<NodeRequire> for NodeRequireFunction {
    fn from(child: NodeRequire) -> Self {
        child.unchecked_into()
    }
}
impl NodeRequire {
    pub fn to_node_require_function(self) -> NodeRequireFunction {
        self.unchecked_into()
    }
    pub fn as_node_require_function(&self) -> &NodeRequireFunction {
        self.unchecked_ref()
    }
}
