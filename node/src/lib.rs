use wasm_bindgen::{prelude::*, JsCast};

// making js_sys type aliases here make it easier to use in the submodules with:
// use crate::*;
type Array = js_sys::Array;
type Function = js_sys::Function;
type IterableIterator = js_sys::Iterator; // TODO Is this correct?
type Object = js_sys::Object;
type Promise = js_sys::Promise;
type ReadonlySet = js_sys::Set; // Readonly types help TypeScript apps
type Uint8Array = js_sys::Uint8Array;
#[wasm_bindgen]
extern "C" {
    pub type AsyncIterableIterator; // TODO How about AsyncIterator?
    pub type MapConstructor;
    pub type Readable;
    pub type SetConstructor;
    pub type WeakMapConstructor;
    pub type WeakSetConstructor;

    // TODO generate 0 to 7 like https://rustwasm.github.io/wasm-bindgen/api/web_sys/console/fn.log_7.html
    #[wasm_bindgen(method, js_name=log)]
    pub fn log_0(this: &Console, message: &JsValue);
    #[wasm_bindgen(method, js_name=log)]
    pub fn log_1(this: &Console, message: &JsValue, optional_params_1: &JsValue);
    #[wasm_bindgen(method, js_name=log)]
    pub fn log_2(this: &Console, message: &JsValue, optional_params_1: &JsValue, optional_params_2: &JsValue);
}

// globals
// https://nodejs.org/api/globals.html
include!("globals_extern.rs");
include!("globals_help.rs");
pub mod node_js {
    use crate::*;
    use wasm_bindgen::JsCast;
    include!("globals_NodeJS_extern.rs");
    include!("globals_NodeJS_help.rs");
}
include!("process_global_NodeJS_extern.rs");
// include!("process_global_NodeJS_help.rs");

pub mod http {
    use crate::*;
    include!("http_extern.rs");
    pub type RequestListener = Closure<dyn FnMut(IncomingMessage, ServerResponse)>;
    // include!("http_help.rs");
    impl AsRef<OutgoingMessage> for ServerResponse {
        fn as_ref(&self) -> &OutgoingMessage {
            self.unchecked_ref()
        }
    }
    impl AsRef<crate::stream::Writable> for ServerResponse {
        fn as_ref(&self) -> &crate::stream::Writable {
            self.unchecked_ref()
        }
    }
    impl ServerResponse {
        pub fn as_outgoing_message(&self) -> &OutgoingMessage {
            self.unchecked_ref()
        }
    }
    impl ServerResponse {
        pub fn as_stream_writable(&self) -> &crate::stream::Writable {
            self.unchecked_ref()
        }
    }
    impl Server {
        pub fn as_net_server(&self) -> &crate::net::Server {
            self.unchecked_ref()
        }
    }
}

pub mod net {
    use crate::*;
    include!("net_extern.rs");
    // include!("net_help.rs");
}

pub mod stream {
    use crate::*;
    include!("stream_extern.rs");
    // include!("stream_help.rs");
}

// pub mod events {
//     include!("events_extern.rs");
//     // include!("events_help.rs");
// }

// pub mod process {
// }