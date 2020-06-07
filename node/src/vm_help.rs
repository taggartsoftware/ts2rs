// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl BaseOptions {
    pub fn new() -> BaseOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl CompileFunctionOptions {
    pub fn new() -> CompileFunctionOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl Context {
    pub fn new() -> Context {
        JsCast::unchecked_into(Object::new())
    }
}
impl CreateContextOptions {
    pub fn new() -> CreateContextOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl RunningScriptOptions {
    pub fn new() -> RunningScriptOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl Script {
    pub fn new() -> Script {
        JsCast::unchecked_into(Object::new())
    }
}
impl ScriptOptions {
    pub fn new() -> ScriptOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<crate::vm::BaseOptions> for crate::vm::CompileFunctionOptions {
    fn as_ref(&self) -> &crate::vm::BaseOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::vm::CompileFunctionOptions> for crate::vm::BaseOptions {
    fn from(child: crate::vm::CompileFunctionOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<crate::vm::BaseOptions> for crate::vm::RunningScriptOptions {
    fn as_ref(&self) -> &crate::vm::BaseOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::vm::RunningScriptOptions> for crate::vm::BaseOptions {
    fn from(child: crate::vm::RunningScriptOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<crate::vm::BaseOptions> for crate::vm::ScriptOptions {
    fn as_ref(&self) -> &crate::vm::BaseOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::vm::ScriptOptions> for crate::vm::BaseOptions {
    fn from(child: crate::vm::ScriptOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
