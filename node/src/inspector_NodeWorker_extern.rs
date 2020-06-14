// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "inspector")]
extern "C" {
    pub type WorkerID;
    #[doc = "Unique identifier of attached debugging session."]
    pub type SessionID;
    pub type WorkerInfo;
    # [ wasm_bindgen ( method , getter , js_name = workerId ) ]
    pub fn worker_id(this: &WorkerInfo) -> inspector::node_worker::WorkerID;
    # [ wasm_bindgen ( method , setter , js_name = workerId ) ]
    pub fn set_worker_id(this: &WorkerInfo, value: inspector::node_worker::WorkerIDRef);
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &WorkerInfo) -> String;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &WorkerInfo, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn title(this: &WorkerInfo) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &WorkerInfo, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &WorkerInfo) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_url(this: &WorkerInfo, value: &str);
    pub type SendMessageToWorkerParameterType;
    #[wasm_bindgen(method, getter)]
    pub fn message(this: &SendMessageToWorkerParameterType) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_message(this: &SendMessageToWorkerParameterType, value: &str);
    #[doc = "Identifier of the session."]
    # [ wasm_bindgen ( method , getter , js_name = sessionId ) ]
    pub fn session_id(this: &SendMessageToWorkerParameterType)
    -> inspector::node_worker::SessionID;
    # [ wasm_bindgen ( method , setter , js_name = sessionId ) ]
    pub fn set_session_id(
        this: &SendMessageToWorkerParameterType,
        value: inspector::node_worker::SessionIDRef,
    );
    pub type EnableParameterType;
    #[doc = "Whether to new workers should be paused until the frontend sends `Runtime.runIfWaitingForDebugger`"]
    #[doc = "message to run them."]
    # [ wasm_bindgen ( method , getter , js_name = waitForDebuggerOnStart ) ]
    pub fn wait_for_debugger_on_start(this: &EnableParameterType) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = waitForDebuggerOnStart ) ]
    pub fn set_wait_for_debugger_on_start(this: &EnableParameterType, value: bool);
    pub type DetachParameterType;
    # [ wasm_bindgen ( method , getter , js_name = sessionId ) ]
    pub fn session_id(this: &DetachParameterType) -> inspector::node_worker::SessionID;
    # [ wasm_bindgen ( method , setter , js_name = sessionId ) ]
    pub fn set_session_id(this: &DetachParameterType, value: inspector::node_worker::SessionIDRef);
    pub type AttachedToWorkerEventDataType;
    #[doc = "Identifier assigned to the session used to send/receive messages."]
    # [ wasm_bindgen ( method , getter , js_name = sessionId ) ]
    pub fn session_id(this: &AttachedToWorkerEventDataType) -> inspector::node_worker::SessionID;
    # [ wasm_bindgen ( method , setter , js_name = sessionId ) ]
    pub fn set_session_id(
        this: &AttachedToWorkerEventDataType,
        value: inspector::node_worker::SessionIDRef,
    );
    # [ wasm_bindgen ( method , getter , js_name = workerInfo ) ]
    pub fn worker_info(this: &AttachedToWorkerEventDataType) -> inspector::node_worker::WorkerInfo;
    # [ wasm_bindgen ( method , setter , js_name = workerInfo ) ]
    pub fn set_worker_info(
        this: &AttachedToWorkerEventDataType,
        value: &inspector::node_worker::WorkerInfo,
    );
    # [ wasm_bindgen ( method , getter , js_name = waitingForDebugger ) ]
    pub fn waiting_for_debugger(this: &AttachedToWorkerEventDataType) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = waitingForDebugger ) ]
    pub fn set_waiting_for_debugger(this: &AttachedToWorkerEventDataType, value: bool);
    pub type DetachedFromWorkerEventDataType;
    #[doc = "Detached session identifier."]
    # [ wasm_bindgen ( method , getter , js_name = sessionId ) ]
    pub fn session_id(this: &DetachedFromWorkerEventDataType) -> inspector::node_worker::SessionID;
    # [ wasm_bindgen ( method , setter , js_name = sessionId ) ]
    pub fn set_session_id(
        this: &DetachedFromWorkerEventDataType,
        value: inspector::node_worker::SessionIDRef,
    );
    pub type ReceivedMessageFromWorkerEventDataType;
    #[doc = "Identifier of a session which sends a message."]
    # [ wasm_bindgen ( method , getter , js_name = sessionId ) ]
    pub fn session_id(
        this: &ReceivedMessageFromWorkerEventDataType,
    ) -> inspector::node_worker::SessionID;
    # [ wasm_bindgen ( method , setter , js_name = sessionId ) ]
    pub fn set_session_id(
        this: &ReceivedMessageFromWorkerEventDataType,
        value: inspector::node_worker::SessionIDRef,
    );
    #[wasm_bindgen(method, getter)]
    pub fn message(this: &ReceivedMessageFromWorkerEventDataType) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_message(this: &ReceivedMessageFromWorkerEventDataType, value: &str);
}