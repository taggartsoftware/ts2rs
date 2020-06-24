// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "string_decoder")]
extern "C" {
    pub type StringDecoder;
    #[wasm_bindgen(constructor)]
    pub fn new_string_decoder(encoding: Option<&str>) -> StringDecoder;
    #[wasm_bindgen(method)]
    pub fn write(this: &StringDecoder, buffer: &Buffer) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_write(this: &StringDecoder, value: &Function);
    #[wasm_bindgen(method)]
    pub fn end(this: &StringDecoder, buffer: Option<&Buffer>) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_end(this: &StringDecoder, value: &Function);
}
