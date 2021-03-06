// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "tty")]
extern "C" {
    #[wasm_bindgen()]
    pub fn isatty(fd: f64) -> bool;
    pub type ReadStream;
    #[wasm_bindgen(constructor)]
    pub fn new_read_stream(
        fd: f64,
        options: Option<&crate::net::SocketConstructorOpts>,
    ) -> ReadStream;
    # [ wasm_bindgen ( method , getter , js_name = isRaw ) ]
    pub fn is_raw(this: &ReadStream) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = isRaw ) ]
    pub fn set_is_raw(this: &ReadStream, value: bool);
    # [ wasm_bindgen ( method , js_name = setRawMode ) ]
    pub fn set_raw_mode(this: &ReadStream, mode: bool) -> ReadStream;
    # [ wasm_bindgen ( method , setter , js_name = setRawMode ) ]
    pub fn set_set_raw_mode(this: &ReadStream, value: &Function);
    # [ wasm_bindgen ( method , getter , js_name = isTTY ) ]
    pub fn is_tty(this: &ReadStream) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = isTTY ) ]
    pub fn set_is_tty(this: &ReadStream, value: bool);
    #[doc = "-1 - to the left from cursor"]
    #[doc = "  0 - the entire line"]
    #[doc = "  1 - to the right from cursor"]
    pub type Direction;
    pub type WriteStream;
    #[wasm_bindgen(constructor)]
    pub fn new_write_stream(fd: f64) -> WriteStream;
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener(this: &WriteStream, event: &str, listener: &JsValue) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener_2(this: &WriteStream, event: &JsValue, listener: &JsValue) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener_2(this: &WriteStream, value: &Function);
    #[wasm_bindgen(method)]
    pub fn emit(this: &WriteStream, event: &JsValue, args: &Array) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_emit(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit_2(this: &WriteStream, event: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit_2(this: &WriteStream, value: &Function);
    #[wasm_bindgen(method)]
    pub fn on(this: &WriteStream, event: &str, listener: &JsValue) -> WriteStream;
    #[wasm_bindgen(method, setter)]
    pub fn set_on(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on_2(this: &WriteStream, event: &JsValue, listener: &JsValue) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on_2(this: &WriteStream, value: &Function);
    #[wasm_bindgen(method)]
    pub fn once(this: &WriteStream, event: &str, listener: &JsValue) -> WriteStream;
    #[wasm_bindgen(method, setter)]
    pub fn set_once(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once_2(this: &WriteStream, event: &JsValue, listener: &JsValue) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once_2(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener(this: &WriteStream, event: &str, listener: &JsValue) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener_2(
        this: &WriteStream,
        event: &JsValue,
        listener: &JsValue,
    ) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener_2(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener(
        this: &WriteStream,
        event: &str,
        listener: &JsValue,
    ) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener_2(
        this: &WriteStream,
        event: &JsValue,
        listener: &JsValue,
    ) -> WriteStream;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener_2(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = clearLine ) ]
    pub fn clear_line(this: &WriteStream, dir: &Direction, callback: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = clearLine ) ]
    pub fn set_clear_line(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = clearScreenDown ) ]
    pub fn clear_screen_down(this: &WriteStream, callback: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = clearScreenDown ) ]
    pub fn set_clear_screen_down(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = cursorTo ) ]
    pub fn cursor_to(this: &WriteStream, x: f64, y: Option<f64>, callback: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = cursorTo ) ]
    pub fn set_cursor_to(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = cursorTo ) ]
    pub fn cursor_to_2(this: &WriteStream, x: f64, callback: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = cursorTo ) ]
    pub fn set_cursor_to_2(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = moveCursor ) ]
    pub fn move_cursor(this: &WriteStream, dx: f64, dy: f64, callback: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = moveCursor ) ]
    pub fn set_move_cursor(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = getColorDepth ) ]
    pub fn get_color_depth(this: &WriteStream, env: &JsValue) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = getColorDepth ) ]
    pub fn set_get_color_depth(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = hasColors ) ]
    pub fn has_colors(this: &WriteStream, depth: Option<f64>) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = hasColors ) ]
    pub fn set_has_colors(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = hasColors ) ]
    pub fn has_colors_2(this: &WriteStream, env: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = hasColors ) ]
    pub fn set_has_colors_2(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = hasColors ) ]
    pub fn has_colors_3(this: &WriteStream, depth: f64, env: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = hasColors ) ]
    pub fn set_has_colors_3(this: &WriteStream, value: &Function);
    # [ wasm_bindgen ( method , js_name = getWindowSize ) ]
    pub fn get_window_size(this: &WriteStream) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = getWindowSize ) ]
    pub fn set_get_window_size(this: &WriteStream, value: &Function);
    #[wasm_bindgen(method, getter)]
    pub fn columns(this: &WriteStream) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_columns(this: &WriteStream, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn rows(this: &WriteStream) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_rows(this: &WriteStream, value: f64);
    # [ wasm_bindgen ( method , getter , js_name = isTTY ) ]
    pub fn is_tty(this: &WriteStream) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = isTTY ) ]
    pub fn set_is_tty(this: &WriteStream, value: bool);
}
