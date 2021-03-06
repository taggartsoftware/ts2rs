// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl CustomPromisify {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl EncodeIntoResult {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl InspectOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl TextDecoder {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl TextEncoder {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<Function> for CustomPromisify {
    fn as_ref(&self) -> &Function {
        self.unchecked_ref()
    }
}
impl From<CustomPromisify> for Function {
    fn from(child: CustomPromisify) -> Self {
        child.unchecked_into()
    }
}
impl CustomPromisify {
    pub fn to_function(self) -> Function {
        self.unchecked_into()
    }
    pub fn as_function(&self) -> &Function {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::InspectOptions> for InspectOptions {
    fn as_ref(&self) -> &node_js::InspectOptions {
        self.unchecked_ref()
    }
}
impl From<InspectOptions> for node_js::InspectOptions {
    fn from(child: InspectOptions) -> Self {
        child.unchecked_into()
    }
}
impl InspectOptions {
    pub fn to_node_js_inspect_options(self) -> node_js::InspectOptions {
        self.unchecked_into()
    }
    pub fn as_node_js_inspect_options(&self) -> &node_js::InspectOptions {
        self.unchecked_ref()
    }
}
