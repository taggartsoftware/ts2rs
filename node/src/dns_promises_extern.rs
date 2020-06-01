// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "promises")]
extern "C" {
    # [ wasm_bindgen ( js_name = getServers ) ]
    pub fn get_servers() -> Array;
    #[wasm_bindgen()]
    pub fn lookup(hostname: &str, family: f64) -> Promise;
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup2(hostname: &str, options: &LookupOneOptions) -> Promise;
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup3(hostname: &str, options: &LookupAllOptions) -> Promise;
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup4(hostname: &str, options: &LookupOptions) -> Promise;
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup5(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = lookupService ) ]
    pub fn lookup_service(address: &str, port: f64) -> Promise;
    #[wasm_bindgen()]
    pub fn resolve(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve2(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve3(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve4(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve5(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve6(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve7(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve8(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve9(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve10(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve11(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve12(hostname: &str, rrtype: &JsValue) -> Promise;
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve13(hostname: &str, rrtype: &str) -> Promise;
    #[wasm_bindgen()]
    pub fn resolve4(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolve4 ) ]
    pub fn resolve42(hostname: &str, options: &ResolveWithTtlOptions) -> Promise;
    # [ wasm_bindgen ( js_name = resolve4 ) ]
    pub fn resolve43(hostname: &str, options: &ResolveOptions) -> Promise;
    #[wasm_bindgen()]
    pub fn resolve6(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolve6 ) ]
    pub fn resolve62(hostname: &str, options: &ResolveWithTtlOptions) -> Promise;
    # [ wasm_bindgen ( js_name = resolve6 ) ]
    pub fn resolve63(hostname: &str, options: &ResolveOptions) -> Promise;
    # [ wasm_bindgen ( js_name = resolveAny ) ]
    pub fn resolve_any(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveCname ) ]
    pub fn resolve_cname(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveMx ) ]
    pub fn resolve_mx(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveNaptr ) ]
    pub fn resolve_naptr(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveNs ) ]
    pub fn resolve_ns(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolvePtr ) ]
    pub fn resolve_ptr(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveSoa ) ]
    pub fn resolve_soa(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveSrv ) ]
    pub fn resolve_srv(hostname: &str) -> Promise;
    # [ wasm_bindgen ( js_name = resolveTxt ) ]
    pub fn resolve_txt(hostname: &str) -> Promise;
    #[wasm_bindgen()]
    pub fn reverse(ip: &str) -> Promise;
    # [ wasm_bindgen ( js_name = setServers ) ]
    pub fn set_servers(servers: &ReadonlyArray);
    pub type Resolver;
    # [ wasm_bindgen ( method , getter , js_name = getServers ) ]
    pub fn get_servers(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = getServers ) ]
    pub fn set_get_servers(this: &Resolver, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn resolve(this: &Resolver) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_resolve(this: &Resolver, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn resolve4(this: &Resolver) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_resolve4(this: &Resolver, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn resolve6(this: &Resolver) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_resolve6(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveAny ) ]
    pub fn resolve_any(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveAny ) ]
    pub fn set_resolve_any(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveCname ) ]
    pub fn resolve_cname(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveCname ) ]
    pub fn set_resolve_cname(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveMx ) ]
    pub fn resolve_mx(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveMx ) ]
    pub fn set_resolve_mx(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveNaptr ) ]
    pub fn resolve_naptr(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveNaptr ) ]
    pub fn set_resolve_naptr(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveNs ) ]
    pub fn resolve_ns(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveNs ) ]
    pub fn set_resolve_ns(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolvePtr ) ]
    pub fn resolve_ptr(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolvePtr ) ]
    pub fn set_resolve_ptr(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveSoa ) ]
    pub fn resolve_soa(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveSoa ) ]
    pub fn set_resolve_soa(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveSrv ) ]
    pub fn resolve_srv(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveSrv ) ]
    pub fn set_resolve_srv(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = resolveTxt ) ]
    pub fn resolve_txt(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = resolveTxt ) ]
    pub fn set_resolve_txt(this: &Resolver, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn reverse(this: &Resolver) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_reverse(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = setServers ) ]
    pub fn set_servers(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = setServers ) ]
    pub fn set_set_servers(this: &Resolver, value: &JsValue);
}
