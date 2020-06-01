// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "writeFile")]
extern "C" {
    #[doc = "Asynchronously writes data to a file, replacing the file if it already exists."]
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify(path: &JsValue, data: &JsValue, options: Option<&WriteFileOptions>)
    -> Promise;
}
