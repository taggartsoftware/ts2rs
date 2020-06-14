// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "dgram")]
extern "C" {
    pub type RemoteInfo;
    #[wasm_bindgen(method, getter)]
    pub fn address(this: &RemoteInfo) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_address(this: &RemoteInfo, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn family(this: &RemoteInfo) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_family(this: &RemoteInfo, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn port(this: &RemoteInfo) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_port(this: &RemoteInfo, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn size(this: &RemoteInfo) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_size(this: &RemoteInfo, value: f64);
    pub type BindOptions;
    #[wasm_bindgen(method, getter)]
    pub fn port(this: &BindOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_port(this: &BindOptions, value: Option<f64>);
    #[wasm_bindgen(method, getter)]
    pub fn address(this: &BindOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_address(this: &BindOptions, value: Option<&str>);
    #[wasm_bindgen(method, getter)]
    pub fn exclusive(this: &BindOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_exclusive(this: &BindOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn fd(this: &BindOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_fd(this: &BindOptions, value: Option<f64>);
    pub type SocketType;
    pub type SocketOptions;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &SocketOptions) -> SocketType;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &SocketOptions, value: &SocketType);
    # [ wasm_bindgen ( method , getter , js_name = reuseAddr ) ]
    pub fn reuse_addr(this: &SocketOptions) -> Option<bool>;
    # [ wasm_bindgen ( method , setter , js_name = reuseAddr ) ]
    pub fn set_reuse_addr(this: &SocketOptions, value: Option<bool>);
    # [ wasm_bindgen ( method , getter , js_name = ipv6Only ) ]
    pub fn ipv6_only(this: &SocketOptions) -> Option<bool>;
    # [ wasm_bindgen ( method , setter , js_name = ipv6Only ) ]
    pub fn set_ipv6_only(this: &SocketOptions, value: Option<bool>);
    # [ wasm_bindgen ( method , getter , js_name = recvBufferSize ) ]
    pub fn recv_buffer_size(this: &SocketOptions) -> Option<f64>;
    # [ wasm_bindgen ( method , setter , js_name = recvBufferSize ) ]
    pub fn set_recv_buffer_size(this: &SocketOptions, value: Option<f64>);
    # [ wasm_bindgen ( method , getter , js_name = sendBufferSize ) ]
    pub fn send_buffer_size(this: &SocketOptions) -> Option<f64>;
    # [ wasm_bindgen ( method , setter , js_name = sendBufferSize ) ]
    pub fn set_send_buffer_size(this: &SocketOptions, value: Option<f64>);
    #[wasm_bindgen(method, getter)]
    pub fn lookup(this: &SocketOptions) -> Option<Function>;
    #[wasm_bindgen(method, setter)]
    pub fn set_lookup(this: &SocketOptions, value: Option<&Function>);
    # [ wasm_bindgen ( js_name = createSocket ) ]
    pub fn create_socket(type_: &SocketType, callback: &JsValue) -> Socket;
    # [ wasm_bindgen ( js_name = createSocket ) ]
    pub fn create_socket2(options: &SocketOptions, callback: &JsValue) -> Socket;
    pub type Socket;
    # [ wasm_bindgen ( method , js_name = addMembership ) ]
    pub fn add_membership(
        this: &Socket,
        multicast_address: &str,
        multicast_interface: Option<&str>,
    );
    # [ wasm_bindgen ( method , setter , js_name = addMembership ) ]
    pub fn set_add_membership(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn address(this: &Socket) -> crate::net::AddressInfo;
    #[wasm_bindgen(method, setter)]
    pub fn set_address(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn bind(this: &Socket, port: Option<f64>, address: Option<&str>, callback: &JsValue);
    #[wasm_bindgen(method, setter)]
    pub fn set_bind(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = bind ) ]
    pub fn bind2(this: &Socket, port: Option<f64>, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = bind ) ]
    pub fn set_bind2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = bind ) ]
    pub fn bind3(this: &Socket, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = bind ) ]
    pub fn set_bind3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = bind ) ]
    pub fn bind4(this: &Socket, options: &BindOptions, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = bind ) ]
    pub fn set_bind4(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn close(this: &Socket, callback: &JsValue);
    #[wasm_bindgen(method, setter)]
    pub fn set_close(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn connect(this: &Socket, port: f64, address: Option<&str>, callback: &JsValue);
    #[wasm_bindgen(method, setter)]
    pub fn set_connect(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = connect ) ]
    pub fn connect2(this: &Socket, port: f64, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = connect ) ]
    pub fn set_connect2(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Socket);
    #[wasm_bindgen(method, setter)]
    pub fn set_disconnect(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = dropMembership ) ]
    pub fn drop_membership(
        this: &Socket,
        multicast_address: &str,
        multicast_interface: Option<&str>,
    );
    # [ wasm_bindgen ( method , setter , js_name = dropMembership ) ]
    pub fn set_drop_membership(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = getRecvBufferSize ) ]
    pub fn get_recv_buffer_size(this: &Socket) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = getRecvBufferSize ) ]
    pub fn set_get_recv_buffer_size(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = getSendBufferSize ) ]
    pub fn get_send_buffer_size(this: &Socket) -> f64;
    # [ wasm_bindgen ( method , setter , js_name = getSendBufferSize ) ]
    pub fn set_get_send_buffer_size(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = ref ) ]
    pub fn ref_(this: &Socket) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = ref ) ]
    pub fn set_ref_(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = remoteAddress ) ]
    pub fn remote_address(this: &Socket) -> crate::net::AddressInfo;
    # [ wasm_bindgen ( method , setter , js_name = remoteAddress ) ]
    pub fn set_remote_address(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn send(
        this: &Socket,
        msg: &JsValue,
        port: Option<f64>,
        address: Option<&str>,
        callback: &JsValue,
    );
    #[wasm_bindgen(method, setter)]
    pub fn set_send(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = send ) ]
    pub fn send2(this: &Socket, msg: &JsValue, port: Option<f64>, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = send ) ]
    pub fn set_send2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = send ) ]
    pub fn send3(this: &Socket, msg: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = send ) ]
    pub fn set_send3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = send ) ]
    pub fn send4(
        this: &Socket,
        msg: &JsValue,
        offset: f64,
        length: f64,
        port: Option<f64>,
        address: Option<&str>,
        callback: &JsValue,
    );
    # [ wasm_bindgen ( method , setter , js_name = send ) ]
    pub fn set_send4(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = send ) ]
    pub fn send5(
        this: &Socket,
        msg: &JsValue,
        offset: f64,
        length: f64,
        port: Option<f64>,
        callback: &JsValue,
    );
    # [ wasm_bindgen ( method , setter , js_name = send ) ]
    pub fn set_send5(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = send ) ]
    pub fn send6(this: &Socket, msg: &JsValue, offset: f64, length: f64, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = send ) ]
    pub fn set_send6(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setBroadcast ) ]
    pub fn set_broadcast(this: &Socket, flag: bool);
    # [ wasm_bindgen ( method , setter , js_name = setBroadcast ) ]
    pub fn set_set_broadcast(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setMulticastInterface ) ]
    pub fn set_multicast_interface(this: &Socket, multicast_interface: &str);
    # [ wasm_bindgen ( method , setter , js_name = setMulticastInterface ) ]
    pub fn set_set_multicast_interface(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setMulticastLoopback ) ]
    pub fn set_multicast_loopback(this: &Socket, flag: bool);
    # [ wasm_bindgen ( method , setter , js_name = setMulticastLoopback ) ]
    pub fn set_set_multicast_loopback(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setMulticastTTL ) ]
    pub fn set_multicast_ttl(this: &Socket, ttl: f64);
    # [ wasm_bindgen ( method , setter , js_name = setMulticastTTL ) ]
    pub fn set_set_multicast_ttl(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setRecvBufferSize ) ]
    pub fn set_recv_buffer_size(this: &Socket, size: f64);
    # [ wasm_bindgen ( method , setter , js_name = setRecvBufferSize ) ]
    pub fn set_set_recv_buffer_size(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setSendBufferSize ) ]
    pub fn set_send_buffer_size(this: &Socket, size: f64);
    # [ wasm_bindgen ( method , setter , js_name = setSendBufferSize ) ]
    pub fn set_set_send_buffer_size(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = setTTL ) ]
    pub fn set_ttl(this: &Socket, ttl: f64);
    # [ wasm_bindgen ( method , setter , js_name = setTTL ) ]
    pub fn set_set_ttl(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn unref(this: &Socket) -> Socket;
    #[wasm_bindgen(method, setter)]
    pub fn set_unref(this: &Socket, value: &Function);
    #[doc = "events.EventEmitter"]
    #[doc = "1. close"]
    #[doc = "2. connect"]
    #[doc = "3. error"]
    #[doc = "4. listening"]
    #[doc = "5. message"]
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener(this: &Socket, event: &str, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener(this: &Socket, value: &Function);
    #[doc = "events.EventEmitter"]
    #[doc = "1. close"]
    #[doc = "2. connect"]
    #[doc = "3. error"]
    #[doc = "4. listening"]
    #[doc = "5. message"]
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener2(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener2(this: &Socket, value: &Function);
    #[doc = "events.EventEmitter"]
    #[doc = "1. close"]
    #[doc = "2. connect"]
    #[doc = "3. error"]
    #[doc = "4. listening"]
    #[doc = "5. message"]
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener3(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener3(this: &Socket, value: &Function);
    #[doc = "events.EventEmitter"]
    #[doc = "1. close"]
    #[doc = "2. connect"]
    #[doc = "3. error"]
    #[doc = "4. listening"]
    #[doc = "5. message"]
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener4(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener4(this: &Socket, value: &Function);
    #[doc = "events.EventEmitter"]
    #[doc = "1. close"]
    #[doc = "2. connect"]
    #[doc = "3. error"]
    #[doc = "4. listening"]
    #[doc = "5. message"]
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener5(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener5(this: &Socket, value: &Function);
    #[doc = "events.EventEmitter"]
    #[doc = "1. close"]
    #[doc = "2. connect"]
    #[doc = "3. error"]
    #[doc = "4. listening"]
    #[doc = "5. message"]
    # [ wasm_bindgen ( method , js_name = addListener ) ]
    pub fn add_listener6(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = addListener ) ]
    pub fn set_add_listener6(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn emit(this: &Socket, event: &JsValue, args: &Array) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_emit(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit2(this: &Socket, event: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit3(this: &Socket, event: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit4(this: &Socket, event: &JsValue, err: &Error) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit4(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit5(this: &Socket, event: &JsValue) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit5(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = emit ) ]
    pub fn emit6(this: &Socket, event: &JsValue, msg: &Buffer, rinfo: &RemoteInfo) -> bool;
    # [ wasm_bindgen ( method , setter , js_name = emit ) ]
    pub fn set_emit6(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn on(this: &Socket, event: &str, listener: &JsValue) -> Socket;
    #[wasm_bindgen(method, setter)]
    pub fn set_on(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on2(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on3(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on4(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on4(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on5(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on5(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = on ) ]
    pub fn on6(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = on ) ]
    pub fn set_on6(this: &Socket, value: &Function);
    #[wasm_bindgen(method)]
    pub fn once(this: &Socket, event: &str, listener: &JsValue) -> Socket;
    #[wasm_bindgen(method, setter)]
    pub fn set_once(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once2(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once3(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once4(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once4(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once5(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once5(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = once ) ]
    pub fn once6(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = once ) ]
    pub fn set_once6(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener(this: &Socket, event: &str, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener2(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener3(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener4(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener4(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener5(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener5(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependListener ) ]
    pub fn prepend_listener6(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependListener ) ]
    pub fn set_prepend_listener6(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener(this: &Socket, event: &str, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener2(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener2(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener3(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener3(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener4(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener4(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener5(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener5(this: &Socket, value: &Function);
    # [ wasm_bindgen ( method , js_name = prependOnceListener ) ]
    pub fn prepend_once_listener6(this: &Socket, event: &JsValue, listener: &JsValue) -> Socket;
    # [ wasm_bindgen ( method , setter , js_name = prependOnceListener ) ]
    pub fn set_prepend_once_listener6(this: &Socket, value: &Function);
}
