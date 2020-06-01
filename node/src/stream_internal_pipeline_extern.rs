// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "pipeline")]
extern "C" {
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify(stream1: &ReadableStream, stream2: &WritableStream) -> Promise;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_2(
        stream1: &ReadableStream,
        stream2: &ReadWriteStream,
        stream3: &WritableStream,
    ) -> Promise;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_3(
        stream1: &ReadableStream,
        stream2: &ReadWriteStream,
        stream3: &ReadWriteStream,
        stream4: &WritableStream,
    ) -> Promise;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_4(
        stream1: &ReadableStream,
        stream2: &ReadWriteStream,
        stream3: &ReadWriteStream,
        stream4: &ReadWriteStream,
        stream5: &WritableStream,
    ) -> Promise;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_5(streams: &Array) -> Promise;
    # [ wasm_bindgen ( js_name = __promisify__ ) ]
    pub fn promisify_6(stream1: &ReadableStream, stream2: &JsValue, streams: &Array) -> Promise;
}
