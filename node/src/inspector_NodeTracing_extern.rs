// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "NodeTracing")]
extern "C" {
    pub type TraceConfig;
    #[doc = "Controls how the trace buffer stores data."]
    # [ wasm_bindgen ( method , getter , js_name = recordMode ) ]
    pub fn record_mode(this: &TraceConfig) -> Option<String>;
    # [ wasm_bindgen ( method , setter , js_name = recordMode ) ]
    pub fn set_record_mode(this: &TraceConfig, value: Option<&str>);
    #[doc = "Included category filters."]
    # [ wasm_bindgen ( method , getter , js_name = includedCategories ) ]
    pub fn included_categories(this: &TraceConfig) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = includedCategories ) ]
    pub fn set_included_categories(this: &TraceConfig, value: &Array);
    pub type StartParameterType;
    # [ wasm_bindgen ( method , getter , js_name = traceConfig ) ]
    pub fn trace_config(this: &StartParameterType) -> TraceConfig;
    # [ wasm_bindgen ( method , setter , js_name = traceConfig ) ]
    pub fn set_trace_config(this: &StartParameterType, value: &TraceConfig);
    pub type GetCategoriesReturnType;
    #[doc = "A list of supported tracing categories."]
    #[wasm_bindgen(method, getter)]
    pub fn categories(this: &GetCategoriesReturnType) -> Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_categories(this: &GetCategoriesReturnType, value: &Array);
    pub type DataCollectedEventDataType;
    #[wasm_bindgen(method, getter)]
    pub fn value(this: &DataCollectedEventDataType) -> Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &DataCollectedEventDataType, value: &Array);
}
