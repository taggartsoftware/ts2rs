// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "http2")]
extern "C" {
    pub static NGHTTP2_SESSION_SERVER: f64;
    pub static NGHTTP2_SESSION_CLIENT: f64;
    pub static NGHTTP2_STREAM_STATE_IDLE: f64;
    pub static NGHTTP2_STREAM_STATE_OPEN: f64;
    pub static NGHTTP2_STREAM_STATE_RESERVED_LOCAL: f64;
    pub static NGHTTP2_STREAM_STATE_RESERVED_REMOTE: f64;
    pub static NGHTTP2_STREAM_STATE_HALF_CLOSED_LOCAL: f64;
    pub static NGHTTP2_STREAM_STATE_HALF_CLOSED_REMOTE: f64;
    pub static NGHTTP2_STREAM_STATE_CLOSED: f64;
    pub static NGHTTP2_NO_ERROR: f64;
    pub static NGHTTP2_PROTOCOL_ERROR: f64;
    pub static NGHTTP2_INTERNAL_ERROR: f64;
    pub static NGHTTP2_FLOW_CONTROL_ERROR: f64;
    pub static NGHTTP2_SETTINGS_TIMEOUT: f64;
    pub static NGHTTP2_STREAM_CLOSED: f64;
    pub static NGHTTP2_FRAME_SIZE_ERROR: f64;
    pub static NGHTTP2_REFUSED_STREAM: f64;
    pub static NGHTTP2_CANCEL: f64;
    pub static NGHTTP2_COMPRESSION_ERROR: f64;
    pub static NGHTTP2_CONNECT_ERROR: f64;
    pub static NGHTTP2_ENHANCE_YOUR_CALM: f64;
    pub static NGHTTP2_INADEQUATE_SECURITY: f64;
    pub static NGHTTP2_HTTP_1_1_REQUIRED: f64;
    pub static NGHTTP2_ERR_FRAME_SIZE_ERROR: f64;
    pub static NGHTTP2_FLAG_NONE: f64;
    pub static NGHTTP2_FLAG_END_STREAM: f64;
    pub static NGHTTP2_FLAG_END_HEADERS: f64;
    pub static NGHTTP2_FLAG_ACK: f64;
    pub static NGHTTP2_FLAG_PADDED: f64;
    pub static NGHTTP2_FLAG_PRIORITY: f64;
    pub static DEFAULT_SETTINGS_HEADER_TABLE_SIZE: f64;
    pub static DEFAULT_SETTINGS_ENABLE_PUSH: f64;
    pub static DEFAULT_SETTINGS_INITIAL_WINDOW_SIZE: f64;
    pub static DEFAULT_SETTINGS_MAX_FRAME_SIZE: f64;
    pub static MAX_MAX_FRAME_SIZE: f64;
    pub static MIN_MAX_FRAME_SIZE: f64;
    pub static MAX_INITIAL_WINDOW_SIZE: f64;
    pub static NGHTTP2_DEFAULT_WEIGHT: f64;
    pub static NGHTTP2_SETTINGS_HEADER_TABLE_SIZE: f64;
    pub static NGHTTP2_SETTINGS_ENABLE_PUSH: f64;
    pub static NGHTTP2_SETTINGS_MAX_CONCURRENT_STREAMS: f64;
    pub static NGHTTP2_SETTINGS_INITIAL_WINDOW_SIZE: f64;
    pub static NGHTTP2_SETTINGS_MAX_FRAME_SIZE: f64;
    pub static NGHTTP2_SETTINGS_MAX_HEADER_LIST_SIZE: f64;
    pub static PADDING_STRATEGY_NONE: f64;
    pub static PADDING_STRATEGY_MAX: f64;
    pub static PADDING_STRATEGY_CALLBACK: f64;
    pub static HTTP2_HEADER_STATUS: String;
    pub static HTTP2_HEADER_METHOD: String;
    pub static HTTP2_HEADER_AUTHORITY: String;
    pub static HTTP2_HEADER_SCHEME: String;
    pub static HTTP2_HEADER_PATH: String;
    pub static HTTP2_HEADER_ACCEPT_CHARSET: String;
    pub static HTTP2_HEADER_ACCEPT_ENCODING: String;
    pub static HTTP2_HEADER_ACCEPT_LANGUAGE: String;
    pub static HTTP2_HEADER_ACCEPT_RANGES: String;
    pub static HTTP2_HEADER_ACCEPT: String;
    pub static HTTP2_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: String;
    pub static HTTP2_HEADER_AGE: String;
    pub static HTTP2_HEADER_ALLOW: String;
    pub static HTTP2_HEADER_AUTHORIZATION: String;
    pub static HTTP2_HEADER_CACHE_CONTROL: String;
    pub static HTTP2_HEADER_CONNECTION: String;
    pub static HTTP2_HEADER_CONTENT_DISPOSITION: String;
    pub static HTTP2_HEADER_CONTENT_ENCODING: String;
    pub static HTTP2_HEADER_CONTENT_LANGUAGE: String;
    pub static HTTP2_HEADER_CONTENT_LENGTH: String;
    pub static HTTP2_HEADER_CONTENT_LOCATION: String;
    pub static HTTP2_HEADER_CONTENT_MD5: String;
    pub static HTTP2_HEADER_CONTENT_RANGE: String;
    pub static HTTP2_HEADER_CONTENT_TYPE: String;
    pub static HTTP2_HEADER_COOKIE: String;
    pub static HTTP2_HEADER_DATE: String;
    pub static HTTP2_HEADER_ETAG: String;
    pub static HTTP2_HEADER_EXPECT: String;
    pub static HTTP2_HEADER_EXPIRES: String;
    pub static HTTP2_HEADER_FROM: String;
    pub static HTTP2_HEADER_HOST: String;
    pub static HTTP2_HEADER_IF_MATCH: String;
    pub static HTTP2_HEADER_IF_MODIFIED_SINCE: String;
    pub static HTTP2_HEADER_IF_NONE_MATCH: String;
    pub static HTTP2_HEADER_IF_RANGE: String;
    pub static HTTP2_HEADER_IF_UNMODIFIED_SINCE: String;
    pub static HTTP2_HEADER_LAST_MODIFIED: String;
    pub static HTTP2_HEADER_LINK: String;
    pub static HTTP2_HEADER_LOCATION: String;
    pub static HTTP2_HEADER_MAX_FORWARDS: String;
    pub static HTTP2_HEADER_PREFER: String;
    pub static HTTP2_HEADER_PROXY_AUTHENTICATE: String;
    pub static HTTP2_HEADER_PROXY_AUTHORIZATION: String;
    pub static HTTP2_HEADER_RANGE: String;
    pub static HTTP2_HEADER_REFERER: String;
    pub static HTTP2_HEADER_REFRESH: String;
    pub static HTTP2_HEADER_RETRY_AFTER: String;
    pub static HTTP2_HEADER_SERVER: String;
    pub static HTTP2_HEADER_SET_COOKIE: String;
    pub static HTTP2_HEADER_STRICT_TRANSPORT_SECURITY: String;
    pub static HTTP2_HEADER_TRANSFER_ENCODING: String;
    pub static HTTP2_HEADER_TE: String;
    pub static HTTP2_HEADER_UPGRADE: String;
    pub static HTTP2_HEADER_USER_AGENT: String;
    pub static HTTP2_HEADER_VARY: String;
    pub static HTTP2_HEADER_VIA: String;
    pub static HTTP2_HEADER_WWW_AUTHENTICATE: String;
    pub static HTTP2_HEADER_HTTP2_SETTINGS: String;
    pub static HTTP2_HEADER_KEEP_ALIVE: String;
    pub static HTTP2_HEADER_PROXY_CONNECTION: String;
    pub static HTTP2_METHOD_ACL: String;
    pub static HTTP2_METHOD_BASELINE_CONTROL: String;
    pub static HTTP2_METHOD_BIND: String;
    pub static HTTP2_METHOD_CHECKIN: String;
    pub static HTTP2_METHOD_CHECKOUT: String;
    pub static HTTP2_METHOD_CONNECT: String;
    pub static HTTP2_METHOD_COPY: String;
    pub static HTTP2_METHOD_DELETE: String;
    pub static HTTP2_METHOD_GET: String;
    pub static HTTP2_METHOD_HEAD: String;
    pub static HTTP2_METHOD_LABEL: String;
    pub static HTTP2_METHOD_LINK: String;
    pub static HTTP2_METHOD_LOCK: String;
    pub static HTTP2_METHOD_MERGE: String;
    pub static HTTP2_METHOD_MKACTIVITY: String;
    pub static HTTP2_METHOD_MKCALENDAR: String;
    pub static HTTP2_METHOD_MKCOL: String;
    pub static HTTP2_METHOD_MKREDIRECTREF: String;
    pub static HTTP2_METHOD_MKWORKSPACE: String;
    pub static HTTP2_METHOD_MOVE: String;
    pub static HTTP2_METHOD_OPTIONS: String;
    pub static HTTP2_METHOD_ORDERPATCH: String;
    pub static HTTP2_METHOD_PATCH: String;
    pub static HTTP2_METHOD_POST: String;
    pub static HTTP2_METHOD_PRI: String;
    pub static HTTP2_METHOD_PROPFIND: String;
    pub static HTTP2_METHOD_PROPPATCH: String;
    pub static HTTP2_METHOD_PUT: String;
    pub static HTTP2_METHOD_REBIND: String;
    pub static HTTP2_METHOD_REPORT: String;
    pub static HTTP2_METHOD_SEARCH: String;
    pub static HTTP2_METHOD_TRACE: String;
    pub static HTTP2_METHOD_UNBIND: String;
    pub static HTTP2_METHOD_UNCHECKOUT: String;
    pub static HTTP2_METHOD_UNLINK: String;
    pub static HTTP2_METHOD_UNLOCK: String;
    pub static HTTP2_METHOD_UPDATE: String;
    pub static HTTP2_METHOD_UPDATEREDIRECTREF: String;
    pub static HTTP2_METHOD_VERSION_CONTROL: String;
    pub static HTTP_STATUS_CONTINUE: f64;
    pub static HTTP_STATUS_SWITCHING_PROTOCOLS: f64;
    pub static HTTP_STATUS_PROCESSING: f64;
    pub static HTTP_STATUS_OK: f64;
    pub static HTTP_STATUS_CREATED: f64;
    pub static HTTP_STATUS_ACCEPTED: f64;
    pub static HTTP_STATUS_NON_AUTHORITATIVE_INFORMATION: f64;
    pub static HTTP_STATUS_NO_CONTENT: f64;
    pub static HTTP_STATUS_RESET_CONTENT: f64;
    pub static HTTP_STATUS_PARTIAL_CONTENT: f64;
    pub static HTTP_STATUS_MULTI_STATUS: f64;
    pub static HTTP_STATUS_ALREADY_REPORTED: f64;
    pub static HTTP_STATUS_IM_USED: f64;
    pub static HTTP_STATUS_MULTIPLE_CHOICES: f64;
    pub static HTTP_STATUS_MOVED_PERMANENTLY: f64;
    pub static HTTP_STATUS_FOUND: f64;
    pub static HTTP_STATUS_SEE_OTHER: f64;
    pub static HTTP_STATUS_NOT_MODIFIED: f64;
    pub static HTTP_STATUS_USE_PROXY: f64;
    pub static HTTP_STATUS_TEMPORARY_REDIRECT: f64;
    pub static HTTP_STATUS_PERMANENT_REDIRECT: f64;
    pub static HTTP_STATUS_BAD_REQUEST: f64;
    pub static HTTP_STATUS_UNAUTHORIZED: f64;
    pub static HTTP_STATUS_PAYMENT_REQUIRED: f64;
    pub static HTTP_STATUS_FORBIDDEN: f64;
    pub static HTTP_STATUS_NOT_FOUND: f64;
    pub static HTTP_STATUS_METHOD_NOT_ALLOWED: f64;
    pub static HTTP_STATUS_NOT_ACCEPTABLE: f64;
    pub static HTTP_STATUS_PROXY_AUTHENTICATION_REQUIRED: f64;
    pub static HTTP_STATUS_REQUEST_TIMEOUT: f64;
    pub static HTTP_STATUS_CONFLICT: f64;
    pub static HTTP_STATUS_GONE: f64;
    pub static HTTP_STATUS_LENGTH_REQUIRED: f64;
    pub static HTTP_STATUS_PRECONDITION_FAILED: f64;
    pub static HTTP_STATUS_PAYLOAD_TOO_LARGE: f64;
    pub static HTTP_STATUS_URI_TOO_LONG: f64;
    pub static HTTP_STATUS_UNSUPPORTED_MEDIA_TYPE: f64;
    pub static HTTP_STATUS_RANGE_NOT_SATISFIABLE: f64;
    pub static HTTP_STATUS_EXPECTATION_FAILED: f64;
    pub static HTTP_STATUS_TEAPOT: f64;
    pub static HTTP_STATUS_MISDIRECTED_REQUEST: f64;
    pub static HTTP_STATUS_UNPROCESSABLE_ENTITY: f64;
    pub static HTTP_STATUS_LOCKED: f64;
    pub static HTTP_STATUS_FAILED_DEPENDENCY: f64;
    pub static HTTP_STATUS_UNORDERED_COLLECTION: f64;
    pub static HTTP_STATUS_UPGRADE_REQUIRED: f64;
    pub static HTTP_STATUS_PRECONDITION_REQUIRED: f64;
    pub static HTTP_STATUS_TOO_MANY_REQUESTS: f64;
    pub static HTTP_STATUS_REQUEST_HEADER_FIELDS_TOO_LARGE: f64;
    pub static HTTP_STATUS_UNAVAILABLE_FOR_LEGAL_REASONS: f64;
    pub static HTTP_STATUS_INTERNAL_SERVER_ERROR: f64;
    pub static HTTP_STATUS_NOT_IMPLEMENTED: f64;
    pub static HTTP_STATUS_BAD_GATEWAY: f64;
    pub static HTTP_STATUS_SERVICE_UNAVAILABLE: f64;
    pub static HTTP_STATUS_GATEWAY_TIMEOUT: f64;
    pub static HTTP_STATUS_HTTP_VERSION_NOT_SUPPORTED: f64;
    pub static HTTP_STATUS_VARIANT_ALSO_NEGOTIATES: f64;
    pub static HTTP_STATUS_INSUFFICIENT_STORAGE: f64;
    pub static HTTP_STATUS_LOOP_DETECTED: f64;
    pub static HTTP_STATUS_BANDWIDTH_LIMIT_EXCEEDED: f64;
    pub static HTTP_STATUS_NOT_EXTENDED: f64;
    pub static HTTP_STATUS_NETWORK_AUTHENTICATION_REQUIRED: f64;
}
