// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "async_hooks")]
extern "C" {
    #[doc = "Returns the asyncId of the current execution context."]
    # [ wasm_bindgen ( js_name = executionAsyncId ) ]
    pub fn execution_async_id() -> f64;
    #[doc = "Returns the ID of the resource responsible for calling the callback that is currently being executed."]
    # [ wasm_bindgen ( js_name = triggerAsyncId ) ]
    pub fn trigger_async_id() -> f64;
    pub type HookCallbacks;
    #[wasm_bindgen(method)]
    pub fn init(
        this: &HookCallbacks,
        async_id: f64,
        type_: &str,
        trigger_async_id: f64,
        resource: &Object,
    );
    #[wasm_bindgen(method, setter)]
    pub fn set_init(this: &HookCallbacks, value: Option<&Function>);
    #[wasm_bindgen(method)]
    pub fn before(this: &HookCallbacks, async_id: f64);
    #[wasm_bindgen(method, setter)]
    pub fn set_before(this: &HookCallbacks, value: Option<&Function>);
    #[wasm_bindgen(method)]
    pub fn after(this: &HookCallbacks, async_id: f64);
    #[wasm_bindgen(method, setter)]
    pub fn set_after(this: &HookCallbacks, value: Option<&Function>);
    # [ wasm_bindgen ( method , js_name = promiseResolve ) ]
    pub fn promise_resolve(this: &HookCallbacks, async_id: f64);
    # [ wasm_bindgen ( method , setter , js_name = promiseResolve ) ]
    pub fn set_promise_resolve(this: &HookCallbacks, value: Option<&Function>);
    #[wasm_bindgen(method)]
    pub fn destroy(this: &HookCallbacks, async_id: f64);
    #[wasm_bindgen(method, setter)]
    pub fn set_destroy(this: &HookCallbacks, value: Option<&Function>);
    pub type AsyncHook;
    #[wasm_bindgen(method)]
    pub fn enable(this: &AsyncHook) -> AsyncHook;
    #[wasm_bindgen(method, setter)]
    pub fn set_enable(this: &AsyncHook, value: &Function);
    #[wasm_bindgen(method)]
    pub fn disable(this: &AsyncHook) -> AsyncHook;
    #[wasm_bindgen(method, setter)]
    pub fn set_disable(this: &AsyncHook, value: &Function);
    #[doc = "Registers functions to be called for different lifetime events of each async operation."]
    # [ wasm_bindgen ( js_name = createHook ) ]
    pub fn create_hook(options: &HookCallbacks) -> AsyncHook;
    pub type AsyncResourceOptions;
    #[doc = "The ID of the execution context that created this async event."]
    #[doc = "Default: `executionAsyncId()`"]
    # [ wasm_bindgen ( method , getter , js_name = triggerAsyncId ) ]
    pub fn trigger_async_id(this: &AsyncResourceOptions) -> Option<f64>;
    # [ wasm_bindgen ( method , setter , js_name = triggerAsyncId ) ]
    pub fn set_trigger_async_id(this: &AsyncResourceOptions, value: Option<f64>);
    #[doc = "Disables automatic `emitDestroy` when the object is garbage collected."]
    #[doc = "This usually does not need to be set (even if `emitDestroy` is called"]
    #[doc = "manually), unless the resource's `asyncId` is retrieved and the"]
    #[doc = "sensitive API's `emitDestroy` is called with it."]
    #[doc = "Default: `false`"]
    # [ wasm_bindgen ( method , getter , js_name = requireManualDestroy ) ]
    pub fn require_manual_destroy(this: &AsyncResourceOptions) -> Option<bool>;
    # [ wasm_bindgen ( method , setter , js_name = requireManualDestroy ) ]
    pub fn set_require_manual_destroy(this: &AsyncResourceOptions, value: Option<bool>);
    #[doc = "The class AsyncResource was designed to be extended by the embedder's async resources."]
    #[doc = "Using this users can easily trigger the lifetime events of their own resources."]
    pub type AsyncResource;
    #[wasm_bindgen(constructor)]
    pub fn new_async_resource(type_: &str, trigger_async_id: &JsValue) -> AsyncResource;
    # [ wasm_bindgen ( method , js_name = runInAsyncScope ) ]
    pub fn run_in_async_scope(
        this: &AsyncResource,
        fn_: &JsValue,
        this_arg: &JsValue,
        args: &Array,
    ) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = runInAsyncScope ) ]
    pub fn set_run_in_async_scope(this: &AsyncResource, value: &Function);
    # [ wasm_bindgen ( method , js_name = emitDestroy ) ]
    pub fn emit_destroy(this: &AsyncResource);
    # [ wasm_bindgen ( method , setter , js_name = emitDestroy ) ]
    pub fn set_emit_destroy(this: &AsyncResource, value: &Function);
    # [ wasm_bindgen ( method , js_name = asyncId ) ]
    pub fn async_id(this: &AsyncResource) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = asyncId ) ]
    pub fn set_async_id(this: &AsyncResource, value: &Function);
    # [ wasm_bindgen ( method , js_name = triggerAsyncId ) ]
    pub fn trigger_async_id(this: &AsyncResource) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = triggerAsyncId ) ]
    pub fn set_trigger_async_id(this: &AsyncResource, value: &Function);
}
