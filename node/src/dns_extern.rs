// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "dns")]
extern "C" {
    pub static ADDRCONFIG: String;
    pub static V4MAPPED: String;
    pub type LookupOptions;
    #[wasm_bindgen(method, getter)]
    pub fn family(this: &LookupOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_family(this: &LookupOptions, value: Option<f64>);
    #[wasm_bindgen(method, getter)]
    pub fn hints(this: &LookupOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_hints(this: &LookupOptions, value: Option<f64>);
    #[wasm_bindgen(method, getter)]
    pub fn all(this: &LookupOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_all(this: &LookupOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn verbatim(this: &LookupOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_verbatim(this: &LookupOptions, value: Option<bool>);
    pub type LookupOneOptions;
    #[wasm_bindgen(method, getter)]
    pub fn all(this: &LookupOneOptions) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_all(this: &LookupOneOptions, value: &JsValue);
    pub type LookupAllOptions;
    #[wasm_bindgen(method, getter)]
    pub fn all(this: &LookupAllOptions) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_all(this: &LookupAllOptions, value: &JsValue);
    pub type LookupAddress;
    #[wasm_bindgen(method, getter)]
    pub fn address(this: &LookupAddress) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_address(this: &LookupAddress, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn family(this: &LookupAddress) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_family(this: &LookupAddress, value: f64);
    #[wasm_bindgen()]
    pub fn lookup(hostname: &str, family: f64, callback: &JsValue);
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup2(hostname: &str, options: &LookupOneOptions, callback: &JsValue);
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup3(hostname: &str, options: &LookupAllOptions, callback: &JsValue);
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup4(hostname: &str, options: &LookupOptions, callback: &JsValue);
    # [ wasm_bindgen ( js_name = lookup ) ]
    pub fn lookup5(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = lookupService ) ]
    pub fn lookup_service(address: &str, port: f64, callback: &JsValue);
    pub type ResolveOptions;
    #[wasm_bindgen(method, getter)]
    pub fn ttl(this: &ResolveOptions) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_ttl(this: &ResolveOptions, value: bool);
    pub type ResolveWithTtlOptions;
    #[wasm_bindgen(method, getter)]
    pub fn ttl(this: &ResolveWithTtlOptions) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_ttl(this: &ResolveWithTtlOptions, value: &JsValue);
    pub type RecordWithTtl;
    #[wasm_bindgen(method, getter)]
    pub fn address(this: &RecordWithTtl) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_address(this: &RecordWithTtl, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn ttl(this: &RecordWithTtl) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_ttl(this: &RecordWithTtl, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn kind(this: &AnyRecordWithTtl) -> i32;
    pub type AnyRecordWithTtl;
    pub type AnyARecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyARecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyARecord, value: &JsValue);
    pub type AnyAaaaRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyAaaaRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyAaaaRecord, value: &JsValue);
    pub type MxRecord;
    #[wasm_bindgen(method, getter)]
    pub fn priority(this: &MxRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_priority(this: &MxRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn exchange(this: &MxRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_exchange(this: &MxRecord, value: &str);
    pub type AnyMxRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyMxRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyMxRecord, value: &JsValue);
    pub type NaptrRecord;
    #[wasm_bindgen(method, getter)]
    pub fn flags(this: &NaptrRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_flags(this: &NaptrRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn service(this: &NaptrRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_service(this: &NaptrRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn regexp(this: &NaptrRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_regexp(this: &NaptrRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn replacement(this: &NaptrRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_replacement(this: &NaptrRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn order(this: &NaptrRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_order(this: &NaptrRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn preference(this: &NaptrRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_preference(this: &NaptrRecord, value: f64);
    pub type AnyNaptrRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyNaptrRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyNaptrRecord, value: &JsValue);
    pub type SoaRecord;
    #[wasm_bindgen(method, getter)]
    pub fn nsname(this: &SoaRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_nsname(this: &SoaRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn hostmaster(this: &SoaRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_hostmaster(this: &SoaRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn serial(this: &SoaRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_serial(this: &SoaRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn refresh(this: &SoaRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_refresh(this: &SoaRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn retry(this: &SoaRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_retry(this: &SoaRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn expire(this: &SoaRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_expire(this: &SoaRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn minttl(this: &SoaRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_minttl(this: &SoaRecord, value: f64);
    pub type AnySoaRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnySoaRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnySoaRecord, value: &JsValue);
    pub type SrvRecord;
    #[wasm_bindgen(method, getter)]
    pub fn priority(this: &SrvRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_priority(this: &SrvRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn weight(this: &SrvRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_weight(this: &SrvRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn port(this: &SrvRecord) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_port(this: &SrvRecord, value: f64);
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &SrvRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &SrvRecord, value: &str);
    pub type AnySrvRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnySrvRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnySrvRecord, value: &JsValue);
    pub type AnyTxtRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyTxtRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyTxtRecord, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn entries(this: &AnyTxtRecord) -> Array;
    #[wasm_bindgen(method, setter)]
    pub fn set_entries(this: &AnyTxtRecord, value: &Array);
    pub type AnyNsRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyNsRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyNsRecord, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn value(this: &AnyNsRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &AnyNsRecord, value: &str);
    pub type AnyPtrRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyPtrRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyPtrRecord, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn value(this: &AnyPtrRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &AnyPtrRecord, value: &str);
    pub type AnyCnameRecord;
    # [ wasm_bindgen ( method , getter , js_name = type ) ]
    pub fn type_(this: &AnyCnameRecord) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = type ) ]
    pub fn set_type_(this: &AnyCnameRecord, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn value(this: &AnyCnameRecord) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &AnyCnameRecord, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn kind(this: &AnyRecord) -> i32;
    pub type AnyRecord;
    #[wasm_bindgen()]
    pub fn resolve(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve2(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve3(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve4(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve5(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve6(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve7(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve8(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve9(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve10(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve11(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve12(hostname: &str, rrtype: &JsValue, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve ) ]
    pub fn resolve13(hostname: &str, rrtype: &str, callback: &JsValue);
    #[wasm_bindgen()]
    pub fn resolve4(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve4 ) ]
    pub fn resolve42(hostname: &str, options: &ResolveWithTtlOptions, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve4 ) ]
    pub fn resolve43(hostname: &str, options: &ResolveOptions, callback: &JsValue);
    #[wasm_bindgen()]
    pub fn resolve6(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve6 ) ]
    pub fn resolve62(hostname: &str, options: &ResolveWithTtlOptions, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolve6 ) ]
    pub fn resolve63(hostname: &str, options: &ResolveOptions, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveCname ) ]
    pub fn resolve_cname(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveMx ) ]
    pub fn resolve_mx(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveNaptr ) ]
    pub fn resolve_naptr(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveNs ) ]
    pub fn resolve_ns(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolvePtr ) ]
    pub fn resolve_ptr(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveSoa ) ]
    pub fn resolve_soa(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveSrv ) ]
    pub fn resolve_srv(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveTxt ) ]
    pub fn resolve_txt(hostname: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = resolveAny ) ]
    pub fn resolve_any(hostname: &str, callback: &JsValue);
    #[wasm_bindgen()]
    pub fn reverse(ip: &str, callback: &JsValue);
    # [ wasm_bindgen ( js_name = setServers ) ]
    pub fn set_servers(servers: &ReadonlyArray);
    # [ wasm_bindgen ( js_name = getServers ) ]
    pub fn get_servers() -> Array;
    pub static NODATA: String;
    pub static FORMERR: String;
    pub static SERVFAIL: String;
    pub static NOTFOUND: String;
    pub static NOTIMP: String;
    pub static REFUSED: String;
    pub static BADQUERY: String;
    pub static BADNAME: String;
    pub static BADFAMILY: String;
    pub static BADRESP: String;
    pub static CONNREFUSED: String;
    pub static TIMEOUT: String;
    pub static EOF: String;
    pub static FILE: String;
    pub static NOMEM: String;
    pub static DESTRUCTION: String;
    pub static BADSTR: String;
    pub static BADFLAGS: String;
    pub static NONAME: String;
    pub static BADHINTS: String;
    pub static NOTINITIALIZED: String;
    pub static LOADIPHLPAPI: String;
    pub static ADDRGETNETWORKPARAMS: String;
    pub static CANCELLED: String;
    pub type Resolver;
    # [ wasm_bindgen ( method , getter , js_name = getServers ) ]
    pub fn get_servers(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = getServers ) ]
    pub fn set_get_servers(this: &Resolver, value: &JsValue);
    # [ wasm_bindgen ( method , getter , js_name = setServers ) ]
    pub fn set_servers(this: &Resolver) -> JsValue;
    # [ wasm_bindgen ( method , setter , js_name = setServers ) ]
    pub fn set_set_servers(this: &Resolver, value: &JsValue);
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
    #[wasm_bindgen(method)]
    pub fn cancel(this: &Resolver);
    #[wasm_bindgen(method, setter)]
    pub fn set_cancel(this: &Resolver, value: &Function);
}
