// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "read")]
extern "C" {
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify(
        fd: f64,
        buffer: &JsValue,
        offset: f64,
        length: f64,
        position: &JsValue,
    ) -> Promise;
}
