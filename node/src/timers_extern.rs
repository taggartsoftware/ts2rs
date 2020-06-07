// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "timers")]
extern "C" {
    # [ wasm_bindgen ( js_name = setTimeout ) ]
    pub fn set_timeout(callback: &JsValue, ms: f64, args: &Array) -> crate::node_js::Timeout;
    # [ wasm_bindgen ( js_name = clearTimeout ) ]
    pub fn clear_timeout(timeout_id: &crate::node_js::Timeout);
    # [ wasm_bindgen ( js_name = setInterval ) ]
    pub fn set_interval(callback: &JsValue, ms: f64, args: &Array) -> crate::node_js::Timeout;
    # [ wasm_bindgen ( js_name = clearInterval ) ]
    pub fn clear_interval(interval_id: &crate::node_js::Timeout);
    # [ wasm_bindgen ( js_name = setImmediate ) ]
    pub fn set_immediate(callback: &JsValue, args: &Array) -> crate::node_js::Immediate;
    # [ wasm_bindgen ( js_name = clearImmediate ) ]
    pub fn clear_immediate(immediate_id: &crate::node_js::Immediate);
}
