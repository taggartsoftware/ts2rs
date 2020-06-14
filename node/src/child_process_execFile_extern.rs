// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "child_process")]
extern "C" {
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify(file: &str) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_2(file: &str, args: &JsValue) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_3(
        file: &str,
        options: &child_process::ExecFileOptionsWithBufferEncoding,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_4(
        file: &str,
        args: &JsValue,
        options: &child_process::ExecFileOptionsWithBufferEncoding,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_5(
        file: &str,
        options: &child_process::ExecFileOptionsWithStringEncoding,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_6(
        file: &str,
        args: &JsValue,
        options: &child_process::ExecFileOptionsWithStringEncoding,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_7(
        file: &str,
        options: &child_process::ExecFileOptionsWithOtherEncoding,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_8(
        file: &str,
        args: &JsValue,
        options: &child_process::ExecFileOptionsWithOtherEncoding,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_9(
        file: &str,
        options: &child_process::ExecFileOptions,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_10(
        file: &str,
        args: &JsValue,
        options: &child_process::ExecFileOptions,
    ) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_11(file: &str, options: &JsValue) -> child_process::PromiseWithChild;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_12(
        file: &str,
        args: &JsValue,
        options: &JsValue,
    ) -> child_process::PromiseWithChild;
}
