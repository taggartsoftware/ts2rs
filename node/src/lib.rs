use wasm_bindgen::{prelude::*, JsCast};

// making js_sys type aliases here make it easier to use in the submodules with:
// use crate::*;

pub type Array = js_sys::Array;
pub type Function = js_sys::Function;
pub type IterableIterator = js_sys::Iterator; // TODO Is this correct?
pub type Object = js_sys::Object;
pub type Promise = js_sys::Promise;
// Readonly types help TypeScript apps
pub type ReadonlyArray = js_sys::Array;
pub type ReadonlySet = js_sys::Set;
pub type Symbol = js_sys::Symbol;
pub type Uint8Array = js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {
    pub type AsyncIterableIterator; // TODO How about AsyncIterator?
    pub type MapConstructor;
    pub type Readable;
    pub type SetConstructor;
    pub type WeakMapConstructor;
    pub type WeakSetConstructor;

    // TODO varargs, https://github.com/taggartsoftware/ts2rs/issues/11
    #[wasm_bindgen(method, js_name=log)]
    pub fn log_va0(this: &Console, message: &JsValue);
    #[wasm_bindgen(method, js_name=log)]
    pub fn log_va1(this: &Console, message: &JsValue, optional_params_1: &JsValue);
    #[wasm_bindgen(method, js_name=log)]
    pub fn log_va2(this: &Console, message: &JsValue, optional_params_1: &JsValue, optional_params_2: &JsValue);
}

// globals
// https://nodejs.org/api/globals.html
include!("globals_extern.rs");
include!("globals_help.rs");
pub mod node_js {
    use crate::*;
    use wasm_bindgen::JsCast;
    include!("globals_NodeJS_alias.rs");
    include!("globals_NodeJS_extern.rs");
    include!("globals_NodeJS_help.rs");

    #[wasm_bindgen]
    extern "C" {
        // TODO support indexing https://github.com/taggartsoftware/ts2rs/issues/10
        #[wasm_bindgen(method, structural, indexing_getter)]
        pub fn get(this: &ProcessEnv, prop: &str) -> Option<String>;
    }
}
include!("process_global_NodeJS_extern.rs");
// include!("process_global_NodeJS_help.rs");

pub mod http {
    use crate::*;
    include!("http_alias.rs");
    include!("http_extern.rs");
    include!("http_help.rs");
    impl ServerResponse {
        // TODO this should be generated, bug https://github.com/taggartsoftware/ts2rs/issues/8
        pub fn as_stream_writable(&self) -> &crate::stream::Writable {
            self.unchecked_ref()
        }
    }
}

pub mod net {
    use crate::*;
    include!("net_alias.rs");
    include!("net_extern.rs");
    include!("net_help.rs");

    // TODO resolve aliases to their actual types, but https://github.com/taggartsoftware/ts2rs/issues/9
    pub type NetServer = Server;
}

pub mod stream {
    use crate::*;
    include!("stream_alias.rs");
    include!("stream_extern.rs");
    include!("stream_help.rs");
}

pub mod dns {
    use crate::*;
    include!("dns_extern.rs");
    include!("dns_help.rs");
}

pub mod events {
    use crate::*;
    include!("events_extern.rs");
    include!("events_help.rs");
}
