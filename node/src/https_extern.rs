// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "https")]
extern "C" {
    pub type ServerOptions;
    pub type RequestOptions;
    pub type AgentOptions;
    #[doc = "If true the server will reject any connection which is not"]
    #[doc = "authorized with the list of supplied CAs. This option only has an"]
    #[doc = "effect if requestCert is true."]
    # [ wasm_bindgen ( method , getter , js_name = rejectUnauthorized ) ]
    pub fn reject_unauthorized(this: &AgentOptions) -> Option<bool>;
    # [ wasm_bindgen ( method , setter , js_name = rejectUnauthorized ) ]
    pub fn set_reject_unauthorized(this: &AgentOptions, value: Option<bool>);
    # [ wasm_bindgen ( method , getter , js_name = maxCachedSessions ) ]
    pub fn max_cached_sessions(this: &AgentOptions) -> Option<f64>;
    # [ wasm_bindgen ( method , setter , js_name = maxCachedSessions ) ]
    pub fn set_max_cached_sessions(this: &AgentOptions, value: Option<f64>);
    pub type Agent;
    #[wasm_bindgen(constructor)]
    pub fn new_agent(options: Option<&AgentOptions>) -> Agent;
    #[wasm_bindgen(method, getter)]
    pub fn options(this: &Agent) -> AgentOptions;
    #[wasm_bindgen(method, setter)]
    pub fn set_options(this: &Agent, value: &AgentOptions);
    pub type Server;
    #[wasm_bindgen(constructor)]
    pub fn new_server(request_listener: &crate::http::RequestListener) -> Server;
    #[wasm_bindgen(constructor)]
    pub fn new_server_2(
        options: &ServerOptions,
        request_listener: &crate::http::RequestListener,
    ) -> Server;
    # [ wasm_bindgen ( method , js_name = setTimeout ) ]
    pub fn set_timeout(this: &Server, callback: &JsValue) -> Server;
    # [ wasm_bindgen ( method , setter , js_name = setTimeout ) ]
    pub fn set_set_timeout(this: &Server, value: &Function);
    # [ wasm_bindgen ( method , js_name = setTimeout ) ]
    pub fn set_timeout_2(this: &Server, msecs: Option<f64>, callback: &JsValue) -> Server;
    # [ wasm_bindgen ( method , setter , js_name = setTimeout ) ]
    pub fn set_set_timeout_2(this: &Server, value: &Function);
    #[doc = "Limits maximum incoming headers count. If set to 0, no limit will be applied."]
    # [ wasm_bindgen ( method , getter , js_name = maxHeadersCount ) ]
    pub fn max_headers_count(this: &Server) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = maxHeadersCount ) ]
    pub fn set_max_headers_count(this: &Server, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn timeout(this: &Server) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_timeout_3(this: &Server, value: f64);
    #[doc = "Limit the amount of time the parser will wait to receive the complete HTTP headers."]
    # [ wasm_bindgen ( method , getter , js_name = headersTimeout ) ]
    pub fn headers_timeout(this: &Server) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = headersTimeout ) ]
    pub fn set_headers_timeout(this: &Server, value: f64);
    # [ wasm_bindgen ( method , getter , js_name = keepAliveTimeout ) ]
    pub fn keep_alive_timeout(this: &Server) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = keepAliveTimeout ) ]
    pub fn set_keep_alive_timeout(this: &Server, value: f64);
    # [ wasm_bindgen ( js_name = createServer ) ]
    pub fn create_server(request_listener: &crate::http::RequestListener) -> Server;
    # [ wasm_bindgen ( js_name = createServer ) ]
    pub fn create_server_2(
        options: &ServerOptions,
        request_listener: &crate::http::RequestListener,
    ) -> Server;
    #[wasm_bindgen()]
    pub fn request(options: &JsValue, callback: &JsValue) -> crate::http::ClientRequest;
    # [ wasm_bindgen ( js_name = request ) ]
    pub fn request_2(
        url: &JsValue,
        options: &RequestOptions,
        callback: &JsValue,
    ) -> crate::http::ClientRequest;
    #[wasm_bindgen()]
    pub fn get(options: &JsValue, callback: &JsValue) -> crate::http::ClientRequest;
    # [ wasm_bindgen ( js_name = get ) ]
    pub fn get_2(
        url: &JsValue,
        options: &RequestOptions,
        callback: &JsValue,
    ) -> crate::http::ClientRequest;
    #[wasm_bindgen(js_name = "globalAgent")]
    pub static GLOBAL_AGENT: Agent;
}
