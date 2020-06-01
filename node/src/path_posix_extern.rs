// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "posix")]
extern "C" {
    #[wasm_bindgen()]
    pub fn normalize(p: &str) -> String;
    #[wasm_bindgen()]
    pub fn join(paths: &Array) -> String;
    #[wasm_bindgen()]
    pub fn resolve(path_segments: &Array) -> String;
    # [ wasm_bindgen ( js_name = isAbsolute ) ]
    pub fn is_absolute(p: &str) -> bool;
    #[wasm_bindgen()]
    pub fn relative(from: &str, to: &str) -> String;
    #[wasm_bindgen()]
    pub fn dirname(p: &str) -> String;
    #[wasm_bindgen()]
    pub fn basename(p: &str, ext: Option<&str>) -> String;
    #[wasm_bindgen()]
    pub fn extname(p: &str) -> String;
    #[wasm_bindgen(js_name = "sep")]
    pub static SEP: String;
    #[wasm_bindgen(js_name = "delimiter")]
    pub static DELIMITER: String;
    #[wasm_bindgen()]
    pub fn parse(p: &str) -> ParsedPath;
    #[wasm_bindgen()]
    pub fn format(p_p: &FormatInputPathObject) -> String;
}
