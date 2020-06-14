// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "util")]
extern "C" {
    pub type InspectOptions;
    #[wasm_bindgen()]
    pub fn format(format: &JsValue, param: &Array) -> String;
    # [ wasm_bindgen ( js_name = formatWithOptions ) ]
    pub fn format_with_options(
        inspect_options: &InspectOptions,
        format: &str,
        param: &Array,
    ) -> String;
    #[wasm_bindgen()]
    pub fn log(string: &str);
    #[wasm_bindgen()]
    pub fn inspect(
        object: &JsValue,
        show_hidden: Option<bool>,
        depth: &JsValue,
        color: Option<bool>,
    ) -> String;
    # [ wasm_bindgen ( js_name = inspect ) ]
    pub fn inspect2(object: &JsValue, options: &InspectOptions) -> String;
    # [ wasm_bindgen ( js_name = isArray ) ]
    pub fn is_array(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isRegExp ) ]
    pub fn is_reg_exp(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isDate ) ]
    pub fn is_date(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isError ) ]
    pub fn is_error(object: &JsValue) -> bool;
    #[wasm_bindgen()]
    pub fn inherits(constructor: &JsValue, super_constructor: &JsValue);
    #[wasm_bindgen()]
    pub fn debuglog(key: &str) -> Function;
    # [ wasm_bindgen ( js_name = isBoolean ) ]
    pub fn is_boolean(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isBuffer ) ]
    pub fn is_buffer(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isFunction ) ]
    pub fn is_function(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isNull ) ]
    pub fn is_null(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isNullOrUndefined ) ]
    pub fn is_null_or_undefined(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isNumber ) ]
    pub fn is_number(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isObject ) ]
    pub fn is_object(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isPrimitive ) ]
    pub fn is_primitive(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isString ) ]
    pub fn is_string(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isSymbol ) ]
    pub fn is_symbol(object: &JsValue) -> bool;
    # [ wasm_bindgen ( js_name = isUndefined ) ]
    pub fn is_undefined(object: &JsValue) -> bool;
    #[wasm_bindgen()]
    pub fn deprecate(fn_: &JsValue, message: &str, code: Option<&str>) -> JsValue;
    # [ wasm_bindgen ( js_name = isDeepStrictEqual ) ]
    pub fn is_deep_strict_equal(val1: &JsValue, val2: &JsValue) -> bool;
    pub type CustomPromisify;
    # [ wasm_bindgen ( method , getter , js_name = __promisify__ ) ]
    pub fn promisify(this: &CustomPromisify) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = __promisify__ ) ]
    pub fn set_promisify(this: &CustomPromisify, value: &JsValue);
    #[wasm_bindgen()]
    pub fn callbackify(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify2(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify3(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify4(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify5(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify6(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify7(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify8(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify9(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify10(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify11(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify12(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify13(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = callbackify ) ]
    pub fn callbackify14(fn_: &JsValue) -> Function;
    #[wasm_bindgen()]
    pub fn promisify(fn_: &CustomPromisify) -> JsValue;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify2(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify3(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify4(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify5(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify6(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify7(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify8(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify9(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify10(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify11(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify12(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify13(fn_: &JsValue) -> Function;
    # [ wasm_bindgen ( js_name = promisify ) ]
    pub fn promisify14(fn_: &Function) -> Function;
    pub type TextDecoder;
    #[wasm_bindgen(method, getter)]
    pub fn encoding(this: &TextDecoder) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn fatal(this: &TextDecoder) -> bool;
    # [ wasm_bindgen ( method , getter , js_name = ignoreBOM ) ]
    pub fn ignore_bom(this: &TextDecoder) -> bool;
    #[wasm_bindgen(constructor)]
    pub fn new_text_decoder(encoding: Option<&str>, options: &JsValue) -> TextDecoder;
    #[wasm_bindgen(method)]
    pub fn decode(this: &TextDecoder, input: &JsValue, options: &JsValue) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_decode(this: &TextDecoder, value: &Function);
    pub type EncodeIntoResult;
    #[doc = "The read Unicode code units of input."]
    #[wasm_bindgen(method, getter)]
    pub fn read(this: &EncodeIntoResult) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_read(this: &EncodeIntoResult, value: f64);
    #[doc = "The written UTF-8 bytes of output."]
    #[wasm_bindgen(method, getter)]
    pub fn written(this: &EncodeIntoResult) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_written(this: &EncodeIntoResult, value: f64);
    pub type TextEncoder;
    #[wasm_bindgen(method, getter)]
    pub fn encoding(this: &TextEncoder) -> String;
    #[wasm_bindgen(method)]
    pub fn encode(this: &TextEncoder, input: Option<&str>) -> Uint8Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_encode(this: &TextEncoder, value: &Function);
    # [ wasm_bindgen ( method , js_name = encodeInto ) ]
    pub fn encode_into(this: &TextEncoder, input: &str, output: &Uint8Array) -> EncodeIntoResult;
    # [ wasm_bindgen ( method , setter , js_name = encodeInto ) ]
    pub fn set_encode_into(this: &TextEncoder, value: &Function);
}