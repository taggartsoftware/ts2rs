// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "buffer")]
extern "C" {
    pub static INSPECT_MAX_BYTES: String;
    #[wasm_bindgen(js_name = "kMaxLength")]
    pub static K_MAX_LENGTH: String;
    #[wasm_bindgen(js_name = "kStringMaxLength")]
    pub static K_STRING_MAX_LENGTH: String;
    #[wasm_bindgen(js_name = "constants")]
    pub static CONSTANTS: String;
    #[wasm_bindgen(js_name = "BuffType")]
    pub static BUFF_TYPE: String;
    pub type TranscodeEncoding;
    #[wasm_bindgen()]
    pub fn transcode(
        source: &Uint8Array,
        from_enc: &TranscodeEncoding,
        to_enc: &TranscodeEncoding,
    ) -> Buffer;
    #[wasm_bindgen(js_name = "SlowBuffer")]
    pub static SLOW_BUFFER: String;
}