// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen]
extern "C" {
    #[doc = "Provides access to the browser's debugging console (e.g.\u{a0}the Web Console in Firefox). The specifics of how it works varies\u{a0}from browser to browser, but there is a de facto set of features that are typically provided."]
    pub type Console;
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &Console) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &Console, value: &JsValue);
    #[wasm_bindgen(method)]
    pub fn assert(this: &Console, condition: Option<bool>, message: Option<&str>, data: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_assert(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn clear(this: &Console);
    #[wasm_bindgen(method, setter)]
    pub fn set_clear(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn count(this: &Console, label: Option<&str>);
    #[wasm_bindgen(method, setter)]
    pub fn set_count(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn debug(this: &Console, message: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_debug(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn dir(this: &Console, value: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_dir(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn dirxml(this: &Console, value: &JsValue);
    #[wasm_bindgen(method, setter)]
    pub fn set_dirxml(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn error(this: &Console, message: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_error(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn exception(this: &Console, message: Option<&str>, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_exception(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn group(this: &Console, group_title: Option<&str>, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_group(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = groupCollapsed ) ]
    pub fn group_collapsed(this: &Console, group_title: Option<&str>, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = groupCollapsed ) ]
    pub fn set_group_collapsed(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = groupEnd ) ]
    pub fn group_end(this: &Console);
    # [ wasm_bindgen ( method , setter , js_name = groupEnd ) ]
    pub fn set_group_end(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn info(this: &Console, message: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_info(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn log(this: &Console, message: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_log(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = markTimeline ) ]
    pub fn mark_timeline(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = markTimeline ) ]
    pub fn set_mark_timeline(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn profile(this: &Console, report_name: Option<&str>);
    #[wasm_bindgen(method, setter)]
    pub fn set_profile(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = profileEnd ) ]
    pub fn profile_end(this: &Console, report_name: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = profileEnd ) ]
    pub fn set_profile_end(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn table(this: &Console, tabular_data: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_table(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn time(this: &Console, label: Option<&str>);
    #[wasm_bindgen(method, setter)]
    pub fn set_time(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timeEnd ) ]
    pub fn time_end(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timeEnd ) ]
    pub fn set_time_end(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timeStamp ) ]
    pub fn time_stamp(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timeStamp ) ]
    pub fn set_time_stamp(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn timeline(this: &Console, label: Option<&str>);
    #[wasm_bindgen(method, setter)]
    pub fn set_timeline(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timelineEnd ) ]
    pub fn timeline_end(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timelineEnd ) ]
    pub fn set_timeline_end(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn trace(this: &Console, message: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_trace(this: &Console, value: &Function);
    #[wasm_bindgen(method)]
    pub fn warn(this: &Console, message: &JsValue, optional_params: &Array);
    #[wasm_bindgen(method, setter)]
    pub fn set_warn(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , getter , js_name = Console ) ]
    pub fn console(this: &Console) -> node_js::ConsoleConstructor;
    # [ wasm_bindgen ( method , setter , js_name = Console ) ]
    pub fn set_console(this: &Console, value: &node_js::ConsoleConstructor);
    # [ wasm_bindgen ( method , js_name = assert ) ]
    pub fn assert_2(
        this: &Console,
        value: &JsValue,
        message: Option<&str>,
        optional_params: &Array,
    );
    # [ wasm_bindgen ( method , setter , js_name = assert ) ]
    pub fn set_assert_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = clear ) ]
    pub fn clear_2(this: &Console);
    # [ wasm_bindgen ( method , setter , js_name = clear ) ]
    pub fn set_clear_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = count ) ]
    pub fn count_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = count ) ]
    pub fn set_count_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = countReset ) ]
    pub fn count_reset(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = countReset ) ]
    pub fn set_count_reset(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = debug ) ]
    pub fn debug_2(this: &Console, message: &JsValue, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = debug ) ]
    pub fn set_debug_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = dir ) ]
    pub fn dir_2(this: &Console, obj: &JsValue, options: Option<&node_js::InspectOptions>);
    # [ wasm_bindgen ( method , setter , js_name = dir ) ]
    pub fn set_dir_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = dirxml ) ]
    pub fn dirxml_2(this: &Console, data: &Array);
    # [ wasm_bindgen ( method , setter , js_name = dirxml ) ]
    pub fn set_dirxml_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = error ) ]
    pub fn error_2(this: &Console, message: &JsValue, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = error ) ]
    pub fn set_error_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = group ) ]
    pub fn group_2(this: &Console, label: &Array);
    # [ wasm_bindgen ( method , setter , js_name = group ) ]
    pub fn set_group_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = groupCollapsed ) ]
    pub fn group_collapsed_2(this: &Console, label: &Array);
    # [ wasm_bindgen ( method , setter , js_name = groupCollapsed ) ]
    pub fn set_group_collapsed_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = groupEnd ) ]
    pub fn group_end_2(this: &Console);
    # [ wasm_bindgen ( method , setter , js_name = groupEnd ) ]
    pub fn set_group_end_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = info ) ]
    pub fn info_2(this: &Console, message: &JsValue, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = info ) ]
    pub fn set_info_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = log ) ]
    pub fn log_2(this: &Console, message: &JsValue, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = log ) ]
    pub fn set_log_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = table ) ]
    pub fn table_2(this: &Console, tabular_data: &JsValue, properties: Option<&Array>);
    # [ wasm_bindgen ( method , setter , js_name = table ) ]
    pub fn set_table_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = time ) ]
    pub fn time_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = time ) ]
    pub fn set_time_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timeEnd ) ]
    pub fn time_end_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timeEnd ) ]
    pub fn set_time_end_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timeLog ) ]
    pub fn time_log(this: &Console, label: Option<&str>, data: &Array);
    # [ wasm_bindgen ( method , setter , js_name = timeLog ) ]
    pub fn set_time_log(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = trace ) ]
    pub fn trace_2(this: &Console, message: &JsValue, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = trace ) ]
    pub fn set_trace_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = warn ) ]
    pub fn warn_2(this: &Console, message: &JsValue, optional_params: &Array);
    # [ wasm_bindgen ( method , setter , js_name = warn ) ]
    pub fn set_warn_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = markTimeline ) ]
    pub fn mark_timeline_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = markTimeline ) ]
    pub fn set_mark_timeline_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = profile ) ]
    pub fn profile_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = profile ) ]
    pub fn set_profile_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = profileEnd ) ]
    pub fn profile_end_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = profileEnd ) ]
    pub fn set_profile_end_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timeStamp ) ]
    pub fn time_stamp_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timeStamp ) ]
    pub fn set_time_stamp_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timeline ) ]
    pub fn timeline_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timeline ) ]
    pub fn set_timeline_2(this: &Console, value: &Function);
    # [ wasm_bindgen ( method , js_name = timelineEnd ) ]
    pub fn timeline_end_2(this: &Console, label: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = timelineEnd ) ]
    pub fn set_timeline_end_2(this: &Console, value: &Function);
    pub type Error;
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Error) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &Error, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn message(this: &Error) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_message(this: &Error, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn stack(this: &Error) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_stack(this: &Error, value: Option<&str>);
    # [ wasm_bindgen ( method , getter , js_name = stack ) ]
    pub fn stack_2(this: &Error) -> Option<String>;
    # [ wasm_bindgen ( method , setter , js_name = stack ) ]
    pub fn set_stack_2(this: &Error, value: Option<&str>);
    pub type ErrorConstructor;
    #[wasm_bindgen(method, getter)]
    pub fn prototype(this: &ErrorConstructor) -> Error;
    # [ wasm_bindgen ( method , js_name = captureStackTrace ) ]
    pub fn capture_stack_trace(
        this: &ErrorConstructor,
        target_object: &Object,
        constructor_opt: Option<&Function>,
    );
    # [ wasm_bindgen ( method , setter , js_name = captureStackTrace ) ]
    pub fn set_capture_stack_trace(this: &ErrorConstructor, value: &Function);
    #[doc = "Optional override for formatting stack traces"]
    # [ wasm_bindgen ( method , getter , js_name = prepareStackTrace ) ]
    pub fn prepare_stack_trace(this: &ErrorConstructor) -> Option<Function>;
    # [ wasm_bindgen ( method , setter , js_name = prepareStackTrace ) ]
    pub fn set_prepare_stack_trace(this: &ErrorConstructor, value: Option<&Function>);
    # [ wasm_bindgen ( method , getter , js_name = stackTraceLimit ) ]
    pub fn stack_trace_limit(this: &ErrorConstructor) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = stackTraceLimit ) ]
    pub fn set_stack_trace_limit(this: &ErrorConstructor, value: f64);
    pub type SymbolConstructor;
    #[doc = "A method that returns the default iterator for an object. Called by the semantics of the\r"]
    #[doc = "for-of statement."]
    #[wasm_bindgen(method, getter)]
    pub fn iterator(this: &SymbolConstructor) -> JsValue;
    #[doc = "A reference to the prototype."]
    #[wasm_bindgen(method, getter)]
    pub fn prototype(this: &SymbolConstructor) -> Symbol;
    # [ wasm_bindgen ( method , js_name = for ) ]
    pub fn for_(this: &SymbolConstructor, key: &str) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = for ) ]
    pub fn set_for_(this: &SymbolConstructor, value: &Function);
    # [ wasm_bindgen ( method , js_name = keyFor ) ]
    pub fn key_for(this: &SymbolConstructor, sym: &Symbol) -> Option<String>;
    # [ wasm_bindgen ( method , setter , js_name = keyFor ) ]
    pub fn set_key_for(this: &SymbolConstructor, value: &Function);
    #[doc = "A method that determines if a constructor object recognizes an object as one of the\r"]
    #[doc = "constructor’s instances. Called by the semantics of the instanceof operator."]
    # [ wasm_bindgen ( method , getter , js_name = hasInstance ) ]
    pub fn has_instance(this: &SymbolConstructor) -> JsValue;
    #[doc = "A Boolean value that if true indicates that an object should flatten to its array elements\r"]
    #[doc = "by Array.prototype.concat."]
    # [ wasm_bindgen ( method , getter , js_name = isConcatSpreadable ) ]
    pub fn is_concat_spreadable(this: &SymbolConstructor) -> JsValue;
    #[doc = "A regular expression method that matches the regular expression against a string. Called\r"]
    #[doc = "by the String.prototype.match method."]
    # [ wasm_bindgen ( method , getter , js_name = match ) ]
    pub fn match_(this: &SymbolConstructor) -> JsValue;
    #[doc = "A regular expression method that replaces matched substrings of a string. Called by the\r"]
    #[doc = "String.prototype.replace method."]
    #[wasm_bindgen(method, getter)]
    pub fn replace(this: &SymbolConstructor) -> JsValue;
    #[doc = "A regular expression method that returns the index within a string that matches the\r"]
    #[doc = "regular expression. Called by the String.prototype.search method."]
    #[wasm_bindgen(method, getter)]
    pub fn search(this: &SymbolConstructor) -> JsValue;
    #[doc = "A function valued property that is the constructor function that is used to create\r"]
    #[doc = "derived objects."]
    #[wasm_bindgen(method, getter)]
    pub fn species(this: &SymbolConstructor) -> JsValue;
    #[doc = "A regular expression method that splits a string at the indices that match the regular\r"]
    #[doc = "expression. Called by the String.prototype.split method."]
    #[wasm_bindgen(method, getter)]
    pub fn split(this: &SymbolConstructor) -> JsValue;
    #[doc = "A method that converts an object to a corresponding primitive value.\r"]
    #[doc = "Called by the ToPrimitive abstract operation."]
    # [ wasm_bindgen ( method , getter , js_name = toPrimitive ) ]
    pub fn to_primitive(this: &SymbolConstructor) -> JsValue;
    #[doc = "A String value that is used in the creation of the default string description of an object.\r"]
    #[doc = "Called by the built-in method Object.prototype.toString."]
    # [ wasm_bindgen ( method , getter , js_name = toStringTag ) ]
    pub fn to_string_tag(this: &SymbolConstructor) -> JsValue;
    #[doc = "An Object whose own property names are property names that are excluded from the 'with'\r"]
    #[doc = "environment bindings of the associated objects."]
    #[wasm_bindgen(method, getter)]
    pub fn unscopables(this: &SymbolConstructor) -> JsValue;
    #[doc = "A method that returns the default async iterator for an object. Called by the semantics of\r"]
    #[doc = "the for-await-of statement."]
    # [ wasm_bindgen ( method , getter , js_name = asyncIterator ) ]
    pub fn async_iterator(this: &SymbolConstructor) -> JsValue;
    #[wasm_bindgen(method, getter)]
    pub fn observable(this: &SymbolConstructor) -> JsValue;
    #[doc = "The type of `import.meta`.\r"]
    #[doc = "\r"]
    #[doc = "If you need to declare that a given property exists on `import.meta`,\r"]
    #[doc = "this type may be augmented via interface merging."]
    pub type ImportMeta;
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &ImportMeta) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_url(this: &ImportMeta, value: &str);
    #[wasm_bindgen(js_name = "process")]
    pub static PROCESS: node_js::Process;
    #[wasm_bindgen(js_name = "console")]
    pub static CONSOLE: Console;
    #[wasm_bindgen(js_name = "__filename")]
    pub static FILENAME: String;
    #[wasm_bindgen(js_name = "__dirname")]
    pub static DIRNAME: String;
    # [ wasm_bindgen ( js_name = setTimeout ) ]
    pub fn set_timeout(callback: &JsValue, ms: f64, args: &Array) -> node_js::Timeout;
    # [ wasm_bindgen ( js_name = clearTimeout ) ]
    pub fn clear_timeout(timeout_id: &node_js::Timeout);
    # [ wasm_bindgen ( js_name = setInterval ) ]
    pub fn set_interval(callback: &JsValue, ms: f64, args: &Array) -> node_js::Timeout;
    # [ wasm_bindgen ( js_name = clearInterval ) ]
    pub fn clear_interval(interval_id: &node_js::Timeout);
    # [ wasm_bindgen ( js_name = setImmediate ) ]
    pub fn set_immediate(callback: &JsValue, args: &Array) -> node_js::Immediate;
    # [ wasm_bindgen ( js_name = clearImmediate ) ]
    pub fn clear_immediate(immediate_id: &node_js::Immediate);
    # [ wasm_bindgen ( js_name = queueMicrotask ) ]
    pub fn queue_microtask(callback: &JsValue);
    pub type NodeRequireFunction;
    pub type NodeRequireCache;
    pub type NodeRequire;
    #[wasm_bindgen(method, getter)]
    pub fn resolve(this: &NodeRequire) -> RequireResolve;
    #[wasm_bindgen(method, setter)]
    pub fn set_resolve(this: &NodeRequire, value: &RequireResolve);
    #[wasm_bindgen(method, getter)]
    pub fn cache(this: &NodeRequire) -> NodeRequireCache;
    #[wasm_bindgen(method, setter)]
    pub fn set_cache(this: &NodeRequire, value: &NodeRequireCache);
    #[wasm_bindgen(method, getter)]
    pub fn extensions(this: &NodeRequire) -> NodeExtensions;
    #[wasm_bindgen(method, setter)]
    pub fn set_extensions(this: &NodeRequire, value: &NodeExtensions);
    #[wasm_bindgen(method, getter)]
    pub fn main(this: &NodeRequire) -> Option<NodeModule>;
    #[wasm_bindgen(method, setter)]
    pub fn set_main(this: &NodeRequire, value: Option<&NodeModule>);
    pub type RequireResolve;
    #[wasm_bindgen(method)]
    pub fn paths(this: &RequireResolve, request: &str) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_paths(this: &RequireResolve, value: &Function);
    pub type NodeExtensions;
    #[wasm_bindgen(method, getter)]
    pub fn stringliteral(this: &NodeExtensions) -> Function;
    #[wasm_bindgen(method, setter)]
    pub fn set_stringliteral(this: &NodeExtensions, value: &Function);
    # [ wasm_bindgen ( method , getter , js_name = stringliteral ) ]
    pub fn stringliteral_2(this: &NodeExtensions) -> Function;
    # [ wasm_bindgen ( method , setter , js_name = stringliteral ) ]
    pub fn set_stringliteral_2(this: &NodeExtensions, value: &Function);
    # [ wasm_bindgen ( method , getter , js_name = stringliteral ) ]
    pub fn stringliteral_3(this: &NodeExtensions) -> Function;
    # [ wasm_bindgen ( method , setter , js_name = stringliteral ) ]
    pub fn set_stringliteral_3(this: &NodeExtensions, value: &Function);
    #[wasm_bindgen(js_name = "require")]
    pub static REQUIRE: NodeRequire;
    pub type NodeModule;
    #[wasm_bindgen(method, getter)]
    pub fn exports(this: &NodeModule) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_exports(this: &NodeModule, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn require(this: &NodeModule) -> NodeRequireFunction;
    #[wasm_bindgen(method, setter)]
    pub fn set_require(this: &NodeModule, value: &NodeRequireFunction);
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &NodeModule) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_id(this: &NodeModule, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn filename(this: &NodeModule) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_filename(this: &NodeModule, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn loaded(this: &NodeModule) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_loaded(this: &NodeModule, value: bool);
    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &NodeModule) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_parent(this: &NodeModule, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn children(this: &NodeModule) -> Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_children(this: &NodeModule, value: &Array);
    #[wasm_bindgen(method, getter)]
    pub fn paths(this: &NodeModule) -> Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_paths(this: &NodeModule, value: &Array);
    #[wasm_bindgen(js_name = "module")]
    pub static MODULE: NodeModule;
    #[wasm_bindgen(js_name = "exports")]
    pub static EXPORTS: JsValue;
    pub type BufferEncoding;
    #[doc = "Raw data is stored in instances of the Buffer class."]
    #[doc = "A Buffer is similar to an array of integers but corresponds to a raw memory allocation outside the V8 heap.  A Buffer cannot be resized."]
    #[doc = "Valid string encodings: 'ascii'|'utf8'|'utf16le'|'ucs2'(alias of 'utf16le')|'base64'|'binary'(deprecated)|'hex'"]
    pub type Buffer;
    #[wasm_bindgen(method, getter)]
    pub fn constructor(this: &Buffer) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_constructor(this: &Buffer, value: &JsValue);
    # [ wasm_bindgen ( method , js_name = readBigUInt64BE ) ]
    pub fn read_big_u_int64_be(this: &Buffer, offset: Option<f64>) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = readBigUInt64BE ) ]
    pub fn set_read_big_u_int64_be(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = readBigUInt64LE ) ]
    pub fn read_big_u_int64_le(this: &Buffer, offset: Option<f64>) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = readBigUInt64LE ) ]
    pub fn set_read_big_u_int64_le(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = readBigInt64BE ) ]
    pub fn read_big_int64_be(this: &Buffer, offset: Option<f64>) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = readBigInt64BE ) ]
    pub fn set_read_big_int64_be(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = readBigInt64LE ) ]
    pub fn read_big_int64_le(this: &Buffer, offset: Option<f64>) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = readBigInt64LE ) ]
    pub fn set_read_big_int64_le(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = writeBigInt64BE ) ]
    pub fn write_big_int64_be(this: &Buffer, value: &JsValue, offset: Option<f64>) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = writeBigInt64BE ) ]
    pub fn set_write_big_int64_be(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = writeBigInt64LE ) ]
    pub fn write_big_int64_le(this: &Buffer, value: &JsValue, offset: Option<f64>) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = writeBigInt64LE ) ]
    pub fn set_write_big_int64_le(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = writeBigUInt64BE ) ]
    pub fn write_big_u_int64_be(this: &Buffer, value: &JsValue, offset: Option<f64>) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = writeBigUInt64BE ) ]
    pub fn set_write_big_u_int64_be(this: &Buffer, value: &Function);
    # [ wasm_bindgen ( method , js_name = writeBigUInt64LE ) ]
    pub fn write_big_u_int64_le(this: &Buffer, value: &JsValue, offset: Option<f64>) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = writeBigUInt64LE ) ]
    pub fn set_write_big_u_int64_le(this: &Buffer, value: &Function);
    #[wasm_bindgen(js_name = "global")]
    pub static GLOBAL: JsValue;
}
