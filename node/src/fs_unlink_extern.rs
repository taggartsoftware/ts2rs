// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "unlink")]
extern "C" {
    #[doc = "Asynchronous unlink(2) - delete a name and possibly the file it refers to."]
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify(path: &crate::fs::PathLike) -> Promise;
}
