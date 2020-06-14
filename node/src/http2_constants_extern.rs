// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "http2")]
extern "C" {
    pub static NGHTTP2_SESSION_SERVER: String;
    pub static NGHTTP2_SESSION_CLIENT: String;
    pub static NGHTTP2_STREAM_STATE_IDLE: String;
    pub static NGHTTP2_STREAM_STATE_OPEN: String;
    pub static NGHTTP2_STREAM_STATE_RESERVED_LOCAL: String;
    pub static NGHTTP2_STREAM_STATE_RESERVED_REMOTE: String;
    pub static NGHTTP2_STREAM_STATE_HALF_CLOSED_LOCAL: String;
    pub static NGHTTP2_STREAM_STATE_HALF_CLOSED_REMOTE: String;
    pub static NGHTTP2_STREAM_STATE_CLOSED: String;
    pub static NGHTTP2_NO_ERROR: String;
    pub static NGHTTP2_PROTOCOL_ERROR: String;
    pub static NGHTTP2_INTERNAL_ERROR: String;
    pub static NGHTTP2_FLOW_CONTROL_ERROR: String;
    pub static NGHTTP2_SETTINGS_TIMEOUT: String;
    pub static NGHTTP2_STREAM_CLOSED: String;
    pub static NGHTTP2_FRAME_SIZE_ERROR: String;
    pub static NGHTTP2_REFUSED_STREAM: String;
    pub static NGHTTP2_CANCEL: String;
    pub static NGHTTP2_COMPRESSION_ERROR: String;
    pub static NGHTTP2_CONNECT_ERROR: String;
    pub static NGHTTP2_ENHANCE_YOUR_CALM: String;
    pub static NGHTTP2_INADEQUATE_SECURITY: String;
    pub static NGHTTP2_HTTP_1_1_REQUIRED: String;
    pub static NGHTTP2_ERR_FRAME_SIZE_ERROR: String;
    pub static NGHTTP2_FLAG_NONE: String;
    pub static NGHTTP2_FLAG_END_STREAM: String;
    pub static NGHTTP2_FLAG_END_HEADERS: String;
    pub static NGHTTP2_FLAG_ACK: String;
    pub static NGHTTP2_FLAG_PADDED: String;
    pub static NGHTTP2_FLAG_PRIORITY: String;
    pub static DEFAULT_SETTINGS_HEADER_TABLE_SIZE: String;
    pub static DEFAULT_SETTINGS_ENABLE_PUSH: String;
    pub static DEFAULT_SETTINGS_INITIAL_WINDOW_SIZE: String;
    pub static DEFAULT_SETTINGS_MAX_FRAME_SIZE: String;
    pub static MAX_MAX_FRAME_SIZE: String;
    pub static MIN_MAX_FRAME_SIZE: String;
    pub static MAX_INITIAL_WINDOW_SIZE: String;
    pub static NGHTTP2_DEFAULT_WEIGHT: String;
    pub static NGHTTP2_SETTINGS_HEADER_TABLE_SIZE: String;
    pub static NGHTTP2_SETTINGS_ENABLE_PUSH: String;
    pub static NGHTTP2_SETTINGS_MAX_CONCURRENT_STREAMS: String;
    pub static NGHTTP2_SETTINGS_INITIAL_WINDOW_SIZE: String;
    pub static NGHTTP2_SETTINGS_MAX_FRAME_SIZE: String;
    pub static NGHTTP2_SETTINGS_MAX_HEADER_LIST_SIZE: String;
    pub static PADDING_STRATEGY_NONE: String;
    pub static PADDING_STRATEGY_MAX: String;
    pub static PADDING_STRATEGY_CALLBACK: String;
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
    pub static HTTP_STATUS_CONTINUE: String;
    pub static HTTP_STATUS_SWITCHING_PROTOCOLS: String;
    pub static HTTP_STATUS_PROCESSING: String;
    pub static HTTP_STATUS_OK: String;
    pub static HTTP_STATUS_CREATED: String;
    pub static HTTP_STATUS_ACCEPTED: String;
    pub static HTTP_STATUS_NON_AUTHORITATIVE_INFORMATION: String;
    pub static HTTP_STATUS_NO_CONTENT: String;
    pub static HTTP_STATUS_RESET_CONTENT: String;
    pub static HTTP_STATUS_PARTIAL_CONTENT: String;
    pub static HTTP_STATUS_MULTI_STATUS: String;
    pub static HTTP_STATUS_ALREADY_REPORTED: String;
    pub static HTTP_STATUS_IM_USED: String;
    pub static HTTP_STATUS_MULTIPLE_CHOICES: String;
    pub static HTTP_STATUS_MOVED_PERMANENTLY: String;
    pub static HTTP_STATUS_FOUND: String;
    pub static HTTP_STATUS_SEE_OTHER: String;
    pub static HTTP_STATUS_NOT_MODIFIED: String;
    pub static HTTP_STATUS_USE_PROXY: String;
    pub static HTTP_STATUS_TEMPORARY_REDIRECT: String;
    pub static HTTP_STATUS_PERMANENT_REDIRECT: String;
    pub static HTTP_STATUS_BAD_REQUEST: String;
    pub static HTTP_STATUS_UNAUTHORIZED: String;
    pub static HTTP_STATUS_PAYMENT_REQUIRED: String;
    pub static HTTP_STATUS_FORBIDDEN: String;
    pub static HTTP_STATUS_NOT_FOUND: String;
    pub static HTTP_STATUS_METHOD_NOT_ALLOWED: String;
    pub static HTTP_STATUS_NOT_ACCEPTABLE: String;
    pub static HTTP_STATUS_PROXY_AUTHENTICATION_REQUIRED: String;
    pub static HTTP_STATUS_REQUEST_TIMEOUT: String;
    pub static HTTP_STATUS_CONFLICT: String;
    pub static HTTP_STATUS_GONE: String;
    pub static HTTP_STATUS_LENGTH_REQUIRED: String;
    pub static HTTP_STATUS_PRECONDITION_FAILED: String;
    pub static HTTP_STATUS_PAYLOAD_TOO_LARGE: String;
    pub static HTTP_STATUS_URI_TOO_LONG: String;
    pub static HTTP_STATUS_UNSUPPORTED_MEDIA_TYPE: String;
    pub static HTTP_STATUS_RANGE_NOT_SATISFIABLE: String;
    pub static HTTP_STATUS_EXPECTATION_FAILED: String;
    pub static HTTP_STATUS_TEAPOT: String;
    pub static HTTP_STATUS_MISDIRECTED_REQUEST: String;
    pub static HTTP_STATUS_UNPROCESSABLE_ENTITY: String;
    pub static HTTP_STATUS_LOCKED: String;
    pub static HTTP_STATUS_FAILED_DEPENDENCY: String;
    pub static HTTP_STATUS_UNORDERED_COLLECTION: String;
    pub static HTTP_STATUS_UPGRADE_REQUIRED: String;
    pub static HTTP_STATUS_PRECONDITION_REQUIRED: String;
    pub static HTTP_STATUS_TOO_MANY_REQUESTS: String;
    pub static HTTP_STATUS_REQUEST_HEADER_FIELDS_TOO_LARGE: String;
    pub static HTTP_STATUS_UNAVAILABLE_FOR_LEGAL_REASONS: String;
    pub static HTTP_STATUS_INTERNAL_SERVER_ERROR: String;
    pub static HTTP_STATUS_NOT_IMPLEMENTED: String;
    pub static HTTP_STATUS_BAD_GATEWAY: String;
    pub static HTTP_STATUS_SERVICE_UNAVAILABLE: String;
    pub static HTTP_STATUS_GATEWAY_TIMEOUT: String;
    pub static HTTP_STATUS_HTTP_VERSION_NOT_SUPPORTED: String;
    pub static HTTP_STATUS_VARIANT_ALSO_NEGOTIATES: String;
    pub static HTTP_STATUS_INSUFFICIENT_STORAGE: String;
    pub static HTTP_STATUS_LOOP_DETECTED: String;
    pub static HTTP_STATUS_BANDWIDTH_LIMIT_EXCEEDED: String;
    pub static HTTP_STATUS_NOT_EXTENDED: String;
    pub static HTTP_STATUS_NETWORK_AUTHENTICATION_REQUIRED: String;
}