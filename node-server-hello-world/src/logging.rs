// create a log macro
// based on https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html
#![macro_use]
macro_rules! log {
    ($($t:tt)*) => (node::CONSOLE.log_0(&format_args!($($t)*).to_string().into()))
}
