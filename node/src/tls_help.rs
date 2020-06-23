// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl Certificate {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl CipherNameAndProtocol {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl CommonConnectionOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ConnectionOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl DetailedPeerCertificate {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl EphemeralKeyInfo {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl KeyObject {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl PeerCertificate {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl PxfObject {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SecureContext {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SecureContextOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SecurePair {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Server {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl TLSSocket {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl TLSSocketOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl TlsOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<CommonConnectionOptions> for ConnectionOptions {
    fn as_ref(&self) -> &CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl From<ConnectionOptions> for CommonConnectionOptions {
    fn from(child: ConnectionOptions) -> Self {
        child.unchecked_into()
    }
}
impl ConnectionOptions {
    pub fn to_common_connection_options(self) -> CommonConnectionOptions {
        self.unchecked_into()
    }
    pub fn as_common_connection_options(&self) -> &CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SecureContextOptions> for ConnectionOptions {
    fn as_ref(&self) -> &SecureContextOptions {
        self.unchecked_ref()
    }
}
impl From<ConnectionOptions> for SecureContextOptions {
    fn from(child: ConnectionOptions) -> Self {
        child.unchecked_into()
    }
}
impl ConnectionOptions {
    pub fn to_secure_context_options(self) -> SecureContextOptions {
        self.unchecked_into()
    }
    pub fn as_secure_context_options(&self) -> &SecureContextOptions {
        self.unchecked_ref()
    }
}
impl AsRef<PeerCertificate> for DetailedPeerCertificate {
    fn as_ref(&self) -> &PeerCertificate {
        self.unchecked_ref()
    }
}
impl From<DetailedPeerCertificate> for PeerCertificate {
    fn from(child: DetailedPeerCertificate) -> Self {
        child.unchecked_into()
    }
}
impl DetailedPeerCertificate {
    pub fn to_peer_certificate(self) -> PeerCertificate {
        self.unchecked_into()
    }
    pub fn as_peer_certificate(&self) -> &PeerCertificate {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::EventEmitter> for Server {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<Server> for node_js::EventEmitter {
    fn from(child: Server) -> Self {
        child.unchecked_into()
    }
}
impl Server {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<crate::net::Server> for Server {
    fn as_ref(&self) -> &crate::net::Server {
        self.unchecked_ref()
    }
}
impl From<Server> for crate::net::Server {
    fn from(child: Server) -> Self {
        child.unchecked_into()
    }
}
impl Server {
    pub fn to_net_server(self) -> crate::net::Server {
        self.unchecked_into()
    }
    pub fn as_net_server(&self) -> &crate::net::Server {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::EventEmitter> for TLSSocket {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<TLSSocket> for node_js::EventEmitter {
    fn from(child: TLSSocket) -> Self {
        child.unchecked_into()
    }
}
impl TLSSocket {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::ReadableStream> for TLSSocket {
    fn as_ref(&self) -> &node_js::ReadableStream {
        self.unchecked_ref()
    }
}
impl From<TLSSocket> for node_js::ReadableStream {
    fn from(child: TLSSocket) -> Self {
        child.unchecked_into()
    }
}
impl TLSSocket {
    pub fn to_node_js_readable_stream(self) -> node_js::ReadableStream {
        self.unchecked_into()
    }
    pub fn as_node_js_readable_stream(&self) -> &node_js::ReadableStream {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::WritableStream> for TLSSocket {
    fn as_ref(&self) -> &node_js::WritableStream {
        self.unchecked_ref()
    }
}
impl From<TLSSocket> for node_js::WritableStream {
    fn from(child: TLSSocket) -> Self {
        child.unchecked_into()
    }
}
impl TLSSocket {
    pub fn to_node_js_writable_stream(self) -> node_js::WritableStream {
        self.unchecked_into()
    }
    pub fn as_node_js_writable_stream(&self) -> &node_js::WritableStream {
        self.unchecked_ref()
    }
}
impl AsRef<crate::net::Socket> for TLSSocket {
    fn as_ref(&self) -> &crate::net::Socket {
        self.unchecked_ref()
    }
}
impl From<TLSSocket> for crate::net::Socket {
    fn from(child: TLSSocket) -> Self {
        child.unchecked_into()
    }
}
impl TLSSocket {
    pub fn to_net_socket(self) -> crate::net::Socket {
        self.unchecked_into()
    }
    pub fn as_net_socket(&self) -> &crate::net::Socket {
        self.unchecked_ref()
    }
}
impl AsRef<CommonConnectionOptions> for TLSSocketOptions {
    fn as_ref(&self) -> &CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl From<TLSSocketOptions> for CommonConnectionOptions {
    fn from(child: TLSSocketOptions) -> Self {
        child.unchecked_into()
    }
}
impl TLSSocketOptions {
    pub fn to_common_connection_options(self) -> CommonConnectionOptions {
        self.unchecked_into()
    }
    pub fn as_common_connection_options(&self) -> &CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SecureContextOptions> for TLSSocketOptions {
    fn as_ref(&self) -> &SecureContextOptions {
        self.unchecked_ref()
    }
}
impl From<TLSSocketOptions> for SecureContextOptions {
    fn from(child: TLSSocketOptions) -> Self {
        child.unchecked_into()
    }
}
impl TLSSocketOptions {
    pub fn to_secure_context_options(self) -> SecureContextOptions {
        self.unchecked_into()
    }
    pub fn as_secure_context_options(&self) -> &SecureContextOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonConnectionOptions> for TlsOptions {
    fn as_ref(&self) -> &CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl From<TlsOptions> for CommonConnectionOptions {
    fn from(child: TlsOptions) -> Self {
        child.unchecked_into()
    }
}
impl TlsOptions {
    pub fn to_common_connection_options(self) -> CommonConnectionOptions {
        self.unchecked_into()
    }
    pub fn as_common_connection_options(&self) -> &CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SecureContextOptions> for TlsOptions {
    fn as_ref(&self) -> &SecureContextOptions {
        self.unchecked_ref()
    }
}
impl From<TlsOptions> for SecureContextOptions {
    fn from(child: TlsOptions) -> Self {
        child.unchecked_into()
    }
}
impl TlsOptions {
    pub fn to_secure_context_options(self) -> SecureContextOptions {
        self.unchecked_into()
    }
    pub fn as_secure_context_options(&self) -> &SecureContextOptions {
        self.unchecked_ref()
    }
}
