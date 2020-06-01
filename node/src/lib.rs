// mod console;
// mod fs;

// mod http {
//     use js_sys::{Array, Date, Function, Object, Promise};
//     use wasm_bindgen::{prelude::*, JsCast};
//     type Socket = crate::net::Socket;
//     include!("http_extern.rs");
//     include!("http_help.rs");
// }

mod events {
    use js_sys::{Array, Function, Object, Uint8Array};
    use wasm_bindgen::{prelude::*, JsCast};
    include!("events_extern.rs");
    // include!("events_help.rs");
}

mod net {
    use js_sys::{Array, Function, Object, Uint8Array};
    use wasm_bindgen::{prelude::*, JsCast};
    type Buffer = crate::globals::Buffer;
    type Error = crate::globals::Error;
    include!("net_extern.rs");
    // include!("net_help.rs");
}

mod globals {
    use wasm_bindgen::{prelude::*};
    // globals.d.ts has Buffer
    // https://nodejs.org/api/globals.html
    // TODO why is globals_extern.rs empty ?
    #[wasm_bindgen(module = "globals")]
    extern "C" {
        pub type Buffer;
        pub type Error;
    }
}