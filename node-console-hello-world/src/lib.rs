mod logging;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(argv: &Array) {
    node::CONSOLE.log_va0(argv);

    let args: Vec<String> = argv.iter().flat_map(|v| v.as_string()).collect();
    log!("args: {:?}", args);
}
