// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "url")]
extern "C" {
    pub type UrlObject;
    #[wasm_bindgen(method, getter)]
    pub fn auth(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_auth(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn hash(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_hash(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn host(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_host(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn hostname(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_hostname(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn href(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_href(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn path(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_path(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn pathname(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_pathname(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn protocol(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_protocol(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn search(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_search(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn slashes(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_slashes(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn port(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_port(this: &UrlObject, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn query(this: &UrlObject) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_query(this: &UrlObject, value: &JsValue);
    pub type Url;
    #[wasm_bindgen(method, getter)]
    pub fn auth(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_auth(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn hash(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_hash(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn host(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_host(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn hostname(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_hostname(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn href(this: &Url) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_href(this: &Url, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn path(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_path(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn pathname(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_pathname(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn protocol(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_protocol(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn search(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_search(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn slashes(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_slashes(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn port(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_port(this: &Url, value: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn query(this: &Url) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_query(this: &Url, value: &JsValue);
    pub type UrlWithParsedQuery;
    #[wasm_bindgen(method, getter)]
    pub fn query(this: &UrlWithParsedQuery) -> crate::querystring::ParsedUrlQuery;
    #[wasm_bindgen(method, setter)]
    pub fn set_query(this: &UrlWithParsedQuery, value: &crate::querystring::ParsedUrlQuery);
    pub type UrlWithStringQuery;
    #[wasm_bindgen(method, getter)]
    pub fn query(this: &UrlWithStringQuery) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_query(this: &UrlWithStringQuery, value: &JsValue);
    #[wasm_bindgen()]
    pub fn parse(url_str: &str) -> UrlWithStringQuery;
    # [ wasm_bindgen ( js_name = parse ) ]
    pub fn parse_2(
        url_str: &str,
        parse_query_string: &JsValue,
        slashes_denote_host: Option<bool>,
    ) -> UrlWithStringQuery;
    # [ wasm_bindgen ( js_name = parse ) ]
    pub fn parse_3(
        url_str: &str,
        parse_query_string: &JsValue,
        slashes_denote_host: Option<bool>,
    ) -> UrlWithParsedQuery;
    # [ wasm_bindgen ( js_name = parse ) ]
    pub fn parse_4(
        url_str: &str,
        parse_query_string: bool,
        slashes_denote_host: Option<bool>,
    ) -> Url;
    #[wasm_bindgen()]
    pub fn format(url: &URL, options: Option<&URLFormatOptions>) -> String;
    # [ wasm_bindgen ( js_name = format ) ]
    pub fn format_2(url_object: &JsValue) -> String;
    #[wasm_bindgen()]
    pub fn resolve(from: &str, to: &str) -> String;
    # [ wasm_bindgen ( js_name = domainToASCII ) ]
    pub fn domain_to_ascii(domain: &str) -> String;
    # [ wasm_bindgen ( js_name = domainToUnicode ) ]
    pub fn domain_to_unicode(domain: &str) -> String;
    #[doc = "This function ensures the correct decodings of percent-encoded characters as"]
    #[doc = "well as ensuring a cross-platform valid absolute path string."]
    # [ wasm_bindgen ( js_name = fileURLToPath ) ]
    pub fn file_url_to_path(url: &JsValue) -> String;
    #[doc = "This function ensures that path is resolved absolutely, and that the URL"]
    #[doc = "control characters are correctly encoded when converting into a File URL."]
    # [ wasm_bindgen ( js_name = pathToFileURL ) ]
    pub fn path_to_file_url(url: &str) -> URL;
    pub type URLFormatOptions;
    #[wasm_bindgen(method, getter)]
    pub fn auth(this: &URLFormatOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_auth(this: &URLFormatOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn fragment(this: &URLFormatOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_fragment(this: &URLFormatOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn search(this: &URLFormatOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_search(this: &URLFormatOptions, value: Option<bool>);
    #[wasm_bindgen(method, getter)]
    pub fn unicode(this: &URLFormatOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_unicode(this: &URLFormatOptions, value: Option<bool>);
    pub type URL;
    #[wasm_bindgen(constructor)]
    pub fn new_url(input: &str, base: &JsValue) -> URL;
    #[wasm_bindgen(method, getter)]
    pub fn hash(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_hash(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn host(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_host(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn hostname(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_hostname(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn href(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_href(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn origin(this: &URL) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn password(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_password(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn pathname(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_pathname(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn port(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_port(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn protocol(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_protocol(this: &URL, value: &str);
    #[wasm_bindgen(method, getter)]
    pub fn search(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_search(this: &URL, value: &str);
    # [ wasm_bindgen ( method , getter , js_name = searchParams ) ]
    pub fn search_params(this: &URL) -> URLSearchParams;
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &URL) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_username(this: &URL, value: &str);
    # [ wasm_bindgen ( method , js_name = toString ) ]
    pub fn to_string(this: &URL) -> String;
    # [ wasm_bindgen ( method , setter , js_name = toString ) ]
    pub fn set_to_string(this: &URL, value: &Function);
    # [ wasm_bindgen ( method , js_name = toJSON ) ]
    pub fn to_json(this: &URL) -> String;
    # [ wasm_bindgen ( method , setter , js_name = toJSON ) ]
    pub fn set_to_json(this: &URL, value: &Function);
    pub type URLSearchParams;
    #[wasm_bindgen(constructor)]
    pub fn new_url_search_params(init: &JsValue) -> URLSearchParams;
    #[wasm_bindgen(method)]
    pub fn append(this: &URLSearchParams, name: &str, value: &str);
    #[wasm_bindgen(method, setter)]
    pub fn set_append(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn delete(this: &URLSearchParams, name: &str);
    #[wasm_bindgen(method, setter)]
    pub fn set_delete(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn entries(this: &URLSearchParams) -> IterableIterator;
    #[wasm_bindgen(method, setter)]
    pub fn set_entries(this: &URLSearchParams, value: &Function);
    # [ wasm_bindgen ( method , js_name = forEach ) ]
    pub fn for_each(this: &URLSearchParams, callback: &JsValue);
    # [ wasm_bindgen ( method , setter , js_name = forEach ) ]
    pub fn set_for_each(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn get(this: &URLSearchParams, name: &str) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_get(this: &URLSearchParams, value: &Function);
    # [ wasm_bindgen ( method , js_name = getAll ) ]
    pub fn get_all(this: &URLSearchParams, name: &str) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getAll ) ]
    pub fn set_get_all(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn has(this: &URLSearchParams, name: &str) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_has(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn keys(this: &URLSearchParams) -> IterableIterator;
    #[wasm_bindgen(method, setter)]
    pub fn set_keys(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn set(this: &URLSearchParams, name: &str, value: &str);
    #[wasm_bindgen(method, setter)]
    pub fn set_set(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn sort(this: &URLSearchParams);
    #[wasm_bindgen(method, setter)]
    pub fn set_sort(this: &URLSearchParams, value: &Function);
    # [ wasm_bindgen ( method , js_name = toString ) ]
    pub fn to_string(this: &URLSearchParams) -> String;
    # [ wasm_bindgen ( method , setter , js_name = toString ) ]
    pub fn set_to_string(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn values(this: &URLSearchParams) -> IterableIterator;
    #[wasm_bindgen(method, setter)]
    pub fn set_values(this: &URLSearchParams, value: &Function);
    #[wasm_bindgen(method)]
    pub fn computed_property_name(this: &URLSearchParams) -> IterableIterator;
    #[wasm_bindgen(method, setter)]
    pub fn set_computed_property_name(this: &URLSearchParams, value: &Function);
}
