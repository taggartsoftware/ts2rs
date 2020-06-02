// mod console;
// mod fs;

// mod http {
//     use js_sys::{Array, Date, Function, Object, Promise};
//     use wasm_bindgen::{prelude::*, JsCast};
//     type Socket = crate::net::Socket;
//     include!("http_extern.rs");
//     include!("http_help.rs");
// }

pub mod events {
    use js_sys::{Array, Function, Object, Uint8Array};
    use wasm_bindgen::{prelude::*, JsCast};
    include!("events_extern.rs");
    // include!("events_help.rs");
}

pub mod net {
    use js_sys::{Array, Function, Object, Uint8Array};
    use wasm_bindgen::{prelude::*, JsCast};
    type Buffer = crate::globals::Buffer;
    type Error = crate::globals::Error;
    include!("net_extern.rs");
    // include!("net_help.rs");
}

pub mod globals {
    use js_sys::{Array, Function, Iterator, Object, Uint8Array};
    use wasm_bindgen::{prelude::*};
    use self::nodejs::*;
    type IterableIterator = Iterator; // ?

    // globals.d.ts has Buffer
    // https://nodejs.org/api/globals.html
    // TODO why is globals_extern.rs empty ?
    // #[wasm_bindgen(module = "globals")]
    // extern "C" {
    //     pub type Buffer;
    //     pub type Error;
    // }
    include!("globals_extern.rs");
    // include!("globals_help.rs");

    pub mod nodejs {
        use js_sys::{Array, Function, Iterator, Object, Uint8Array, Promise, AsyncIterator, Set};
        use wasm_bindgen::{prelude::*, JsCast};
        use super::{NodeRequire, Error, NodeRequireFunction, NodeModule, BufferEncoding};
        use crate::process::global::nodejs::{ReadStream, WriteStream};
        type ReadonlySet = Set;
        #[wasm_bindgen(module = "NodeJS")]
        extern "C" {
            pub type Console; // web_sys::console
            pub type MapConstructor;
            pub type SetConstructor;
            pub type WeakMapConstructor;
            pub type WeakSetConstructor;
            pub type AsyncIterableIterator; // AsyncIterator ?
        }
        include!("globals_NodeJS_extern.rs");
        // include!("globals_NodeJS_help.rs");
    }
}

pub mod process {
    pub mod global {
        pub mod nodejs {
            use js_sys::{Object};
            use wasm_bindgen::{prelude::*, JsCast};
            use crate::globals::nodejs::{EventEmitter, ReadableStream, WritableStream};
            #[wasm_bindgen(module = "NodeJS")]
            extern "C" {
                pub type Duplex; // TODO crate::stream::Duplex
                pub type Socket; // TODO
                pub type Readable; // TODO stream
                pub type Streamm; // TODO stream
            }
            include!("process_global_NodeJS_extern.rs");
            // include!("process_global_NodeJS_help.rs");
        }
    }
}