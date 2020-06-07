// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "worker_threads")]
extern "C" {
    #[wasm_bindgen(js_name = "isMainThread")]
    pub static IS_MAIN_THREAD: String;
    #[wasm_bindgen(js_name = "parentPort")]
    pub static PARENT_PORT: String;
    pub static SHARE_ENV: String;
    #[wasm_bindgen(js_name = "threadId")]
    pub static THREAD_ID: String;
    #[wasm_bindgen(js_name = "workerData")]
    pub static WORKER_DATA: String;
    pub type MessageChannel;
    #[wasm_bindgen(method, getter)]
    pub fn port1(this: &MessageChannel) -> crate::worker_threads::MessagePort;
    #[wasm_bindgen(method, getter)]
    pub fn port2(this: &MessageChannel) -> crate::worker_threads::MessagePort;
    pub type MessagePort;
    #[wasm_bindgen(method)]
    pub fn close(this: &MessagePort);
    #[wasm_bindgen(method, setter)]
    pub fn set_close(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = postMessage ) ]
    pub fn post_message(this: &MessagePort, value: &JsValue, transfer_list: Option<&Array>);
    # [ wasm_bindgen ( method , setter , js_name = postMessage ) ]
    pub fn set_post_message(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = ref ) ]
    pub fn ref_(this: &MessagePort);
    # [ wasm_bindgen ( method , setter , js_name = ref ) ]
    pub fn set_ref_(this: &MessagePort, value: &Function);
    #[wasm_bindgen(method)]
    pub fn unref(this: &MessagePort);
    #[wasm_bindgen(method, setter)]
    pub fn set_unref(this: &MessagePort, value: &Function);
    #[wasm_bindgen(method)]
    pub fn start(this: &MessagePort);
    #[wasm_bindgen(method, setter)]
    pub fn set_start(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener2(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener3(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener3(this: &MessagePort, value: &Function);
    #[wasm_bindgen(method)]
    pub fn emit(this: &MessagePort, event: &JsValue) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_emit(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit2(this: &MessagePort, event: &JsValue, value: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit3(this: &MessagePort, event: &JsValue, args: &Array) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit3(this: &MessagePort, value: &Function);
    #[wasm_bindgen(method)]
    pub fn on(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    #[wasm_bindgen(method, setter)]
    pub fn set_on(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on2(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on3(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on3(this: &MessagePort, value: &Function);
    #[wasm_bindgen(method)]
    pub fn once(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    #[wasm_bindgen(method, setter)]
    pub fn set_once(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once2(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once3(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once3(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener(this: &MessagePort, event: &JsValue, listener: &JsValue)
    -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener2(
        this: &MessagePort,
        event: &JsValue,
        listener: &JsValue,
    ) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener3(
        this: &MessagePort,
        event: &JsValue,
        listener: &JsValue,
    ) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener3(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener(
        this: &MessagePort,
        event: &JsValue,
        listener: &JsValue,
    ) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener2(
        this: &MessagePort,
        event: &JsValue,
        listener: &JsValue,
    ) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener3(
        this: &MessagePort,
        event: &JsValue,
        listener: &JsValue,
    ) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener3(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener2(this: &MessagePort, event: &JsValue, listener: &JsValue)
    -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener3(this: &MessagePort, event: &JsValue, listener: &JsValue)
    -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener3(this: &MessagePort, value: &Function);
    #[wasm_bindgen(method)]
    pub fn off(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    #[wasm_bindgen(method, setter)]
    pub fn set_off(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = off ) ]
    pub fn off2(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = off ) ]
    pub fn set_off2(this: &MessagePort, value: &Function);
    # [ wasm_bindgen ( method , js_name = off ) ]
    pub fn off3(this: &MessagePort, event: &JsValue, listener: &JsValue) -> MessagePort;
    # [ wasm_bindgen ( method , setter , js_name = off ) ]
    pub fn set_off3(this: &MessagePort, value: &Function);
    pub type WorkerOptions;
    #[wasm_bindgen(method, getter)]
    pub fn eval(this: &WorkerOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_eval(this: &WorkerOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn env(this: &WorkerOptions) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_env(this: &WorkerOptions, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = workerData ) ]
    pub fn worker_data(this: &WorkerOptions) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = workerData ) ]
    pub fn set_worker_data(this: &WorkerOptions, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn stdin(this: &WorkerOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_stdin(this: &WorkerOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &WorkerOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_stdout(this: &WorkerOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &WorkerOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_stderr(this: &WorkerOptions, value: Option<bool>);
    # [ wasm_bindgen ( method , getter , js_name = execArgv ) ]
    pub fn exec_argv(this: &WorkerOptions) -> Option<Array>;
    # [ wasm_bindgen ( method , setter , js_name = execArgv ) ]
    pub fn set_exec_argv(this: &WorkerOptions, value: Option<&Array>);
    pub type Worker;
    #[wasm_bindgen(method, getter)]
    pub fn stdin(this: &Worker) -> JsValue;
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &Worker) -> Readable;
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &Worker) -> Readable;
    # [ wasm_bindgen ( method , getter , js_name = threadId ) ]
    pub fn thread_id(this: &Worker) -> f64;
    #[wasm_bindgen(constructor)]
    pub fn new_worker(
        filename: &str,
        options: Option<&crate::worker_threads::WorkerOptions>,
    ) -> Worker;
    # [ wasm_bindgen ( method , js_name = postMessage ) ]
    pub fn post_message(this: &Worker, value: &JsValue, transfer_list: Option<&Array>);
    # [ wasm_bindgen ( method , setter , js_name = postMessage ) ]
    pub fn set_post_message(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = ref ) ]
    pub fn ref_(this: &Worker);
    # [ wasm_bindgen ( method , setter , js_name = ref ) ]
    pub fn set_ref_(this: &Worker, value: &Function);
    #[wasm_bindgen(method)]
    pub fn unref(this: &Worker);
    #[wasm_bindgen(method, setter)]
    pub fn set_unref(this: &Worker, value: &Function);
    #[doc = "Stop all JavaScript execution in the worker thread as soon as possible."]
    #[doc = "Returns a Promise for the exit code that is fulfilled when the `exit` event is emitted."]
    #[wasm_bindgen(method)]
    pub fn terminate(this: &Worker) -> Promise;
    #[wasm_bindgen(method, setter)]
    pub fn set_terminate(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener5(this: &Worker, value: &Function);
    #[wasm_bindgen(method)]
    pub fn emit(this: &Worker, event: &JsValue, err: &Error) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_emit(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit2(this: &Worker, event: &JsValue, exit_code: f64) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit3(this: &Worker, event: &JsValue, value: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit4(this: &Worker, event: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit5(this: &Worker, event: &JsValue, args: &Array) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit5(this: &Worker, value: &Function);
    #[wasm_bindgen(method)]
    pub fn on(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    #[wasm_bindgen(method, setter)]
    pub fn set_on(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on5(this: &Worker, value: &Function);
    #[wasm_bindgen(method)]
    pub fn once(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    #[wasm_bindgen(method, setter)]
    pub fn set_once(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once5(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener5(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener5(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = removeListener ) ]
    pub fn remove_listener5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = removeListener ) ]
    pub fn set_remove_listener5(this: &Worker, value: &Function);
    #[wasm_bindgen(method)]
    pub fn off(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    #[wasm_bindgen(method, setter)]
    pub fn set_off(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = off ) ]
    pub fn off2(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = off ) ]
    pub fn set_off2(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = off ) ]
    pub fn off3(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = off ) ]
    pub fn set_off3(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = off ) ]
    pub fn off4(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = off ) ]
    pub fn set_off4(this: &Worker, value: &Function);
    # [ wasm_bindgen ( method , js_name = off ) ]
    pub fn off5(this: &Worker, event: &JsValue, listener: &JsValue) -> Worker;
    # [ wasm_bindgen ( method , setter , js_name = off ) ]
    pub fn set_off5(this: &Worker, value: &Function);
    #[doc = "Transfer a `MessagePort` to a different `vm` Context. The original `port`"]
    #[doc = "object will be rendered unusable, and the returned `MessagePort` instance will"]
    #[doc = "take its place."]
    #[doc = ""]
    #[doc = "The returned `MessagePort` will be an object in the target context, and will"]
    #[doc = "inherit from its global `Object` class. Objects passed to the"]
    #[doc = "`port.onmessage()` listener will also be created in the target context"]
    #[doc = "and inherit from its global `Object` class."]
    #[doc = ""]
    #[doc = "However, the created `MessagePort` will no longer inherit from"]
    #[doc = "`EventEmitter`, and only `port.onmessage()` can be used to receive"]
    #[doc = "events using it."]
    # [ wasm_bindgen ( js_name = moveMessagePortToContext ) ]
    pub fn move_message_port_to_context(
        port: &crate::worker_threads::MessagePort,
        context: &Context,
    ) -> crate::worker_threads::MessagePort;
    #[doc = "Receive a single message from a given `MessagePort`. If no message is available,"]
    #[doc = "`undefined` is returned, otherwise an object with a single `message` property"]
    #[doc = "that contains the message payload, corresponding to the oldest message in the"]
    #[doc = "`MessagePort`’s queue."]
    # [ wasm_bindgen ( js_name = receiveMessageOnPort ) ]
    pub fn receive_message_on_port(port: &crate::worker_threads::MessagePort) -> JsValue;
}
