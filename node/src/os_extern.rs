// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "os")]
extern "C" {
    pub type CpuInfo;
    #[wasm_bindgen(method, getter)]
    pub fn model(this: &CpuInfo) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_model(this: &CpuInfo, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn speed(this: &CpuInfo) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_speed(this: &CpuInfo, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn times(this: &CpuInfo) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_times(this: &CpuInfo, value: &JsValue);
    pub type NetworkInterfaceBase;
    #[wasm_bindgen(method, getter)]
    pub fn address(this: &NetworkInterfaceBase) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_address(this: &NetworkInterfaceBase, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn netmask(this: &NetworkInterfaceBase) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_netmask(this: &NetworkInterfaceBase, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn mac(this: &NetworkInterfaceBase) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_mac(this: &NetworkInterfaceBase, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn internal(this: &NetworkInterfaceBase) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_internal(this: &NetworkInterfaceBase, value: bool);
    #[wasm_bindgen(method, getter)]
    pub fn cidr(this: &NetworkInterfaceBase) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_cidr(this: &NetworkInterfaceBase, value: &JsValue);
    pub type NetworkInterfaceInfoIPv4;
    #[wasm_bindgen(method, getter)]
    pub fn family(this: &NetworkInterfaceInfoIPv4) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_family(this: &NetworkInterfaceInfoIPv4, value: &JsValue);
    pub type NetworkInterfaceInfoIPv6;
    #[wasm_bindgen(method, getter)]
    pub fn family(this: &NetworkInterfaceInfoIPv6) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_family(this: &NetworkInterfaceInfoIPv6, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn scopeid(this: &NetworkInterfaceInfoIPv6) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_scopeid(this: &NetworkInterfaceInfoIPv6, value: f64);
    pub type UserInfo;
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &UserInfo) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_username(this: &UserInfo, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn uid(this: &UserInfo) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_uid(this: &UserInfo, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn gid(this: &UserInfo) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_gid(this: &UserInfo, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn shell(this: &UserInfo) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_shell(this: &UserInfo, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn homedir(this: &UserInfo) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_homedir(this: &UserInfo, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn kind(this: &NetworkInterfaceInfo) -> i32;
    pub type NetworkInterfaceInfo;
    #[wasm_bindgen()]
    pub fn hostname() -> String;
    #[wasm_bindgen()]
    pub fn loadavg() -> Array;
    #[wasm_bindgen()]
    pub fn uptime() -> f64;
    #[wasm_bindgen()]
    pub fn freemem() -> f64;
    #[wasm_bindgen()]
    pub fn totalmem() -> f64;
    #[wasm_bindgen()]
    pub fn cpus() -> Array;
    # [ wasm_bindgen ( js_name = type ) ]
    pub fn type_() -> String;
    #[wasm_bindgen()]
    pub fn release() -> String;
    # [ wasm_bindgen ( js_name = networkInterfaces ) ]
    pub fn network_interfaces() -> JsValue;
    #[wasm_bindgen()]
    pub fn homedir() -> String;
    # [ wasm_bindgen ( js_name = userInfo ) ]
    pub fn user_info(options: &JsValue) -> UserInfo;
    # [ wasm_bindgen ( js_name = userInfo ) ]
    pub fn user_info_2(options: &JsValue) -> UserInfo;
    #[wasm_bindgen(js_name = "constants")]
    pub static CONSTANTS: JsValue;
    #[wasm_bindgen()]
    pub fn arch() -> String;
    #[wasm_bindgen()]
    pub fn platform() -> node_js::Platform;
    #[wasm_bindgen()]
    pub fn tmpdir() -> String;
    pub static EOL: String;
    #[wasm_bindgen()]
    pub fn endianness() -> JsValue;
    #[doc = "Gets the priority of a process."]
    #[doc = "Defaults to current process."]
    # [ wasm_bindgen ( js_name = getPriority ) ]
    pub fn get_priority(pid: Option<f64>) -> f64;
    #[doc = "Sets the priority of the current process."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Sets the priority of the process specified process."]
    # [ wasm_bindgen ( js_name = setPriority ) ]
    pub fn set_priority(priority: f64);
    #[doc = "Sets the priority of the current process."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Sets the priority of the process specified process."]
    # [ wasm_bindgen ( js_name = setPriority ) ]
    pub fn set_priority_2(pid: f64, priority: f64);
}
