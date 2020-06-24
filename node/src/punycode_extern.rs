// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "punycode")]
extern "C" {
    #[wasm_bindgen()]
    pub fn decode(string: &str) -> String;
    #[wasm_bindgen()]
    pub fn encode(string: &str) -> String;
    # [ wasm_bindgen ( js_name = toUnicode ) ]
    pub fn to_unicode(domain: &str) -> String;
    # [ wasm_bindgen ( js_name = toASCII ) ]
    pub fn to_ascii(domain: &str) -> String;
    #[wasm_bindgen(js_name = "ucs2")]
    pub static UCS2: ucs2;
    pub type ucs2;
    #[wasm_bindgen(method)]
    pub fn decode(this: &ucs2, string: &str) -> Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_decode(this: &ucs2, value: &Function);
    #[wasm_bindgen(method)]
    pub fn encode(this: &ucs2, code_points: &Array) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_encode(this: &ucs2, value: &Function);
    #[wasm_bindgen(js_name = "version")]
    pub static VERSION: String;
}
