// create a log macro
// based on https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html
#![macro_use]
#![allow(unused_macros)]
macro_rules! log {
    ($($t:tt)*) => (node::CONSOLE.log_va0(&format_args!($($t)*).to_string().into()))
}
