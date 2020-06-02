// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl Certificate {
    pub fn new() -> Certificate {
        JsCast::unchecked_into(Object::new())
    }
}
impl CipherNameAndProtocol {
    pub fn new() -> CipherNameAndProtocol {
        JsCast::unchecked_into(Object::new())
    }
}
impl CommonConnectionOptions {
    pub fn new() -> CommonConnectionOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl ConnectionOptions {
    pub fn new() -> ConnectionOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl DetailedPeerCertificate {
    pub fn new() -> DetailedPeerCertificate {
        JsCast::unchecked_into(Object::new())
    }
}
impl EphemeralKeyInfo {
    pub fn new() -> EphemeralKeyInfo {
        JsCast::unchecked_into(Object::new())
    }
}
impl KeyObject {
    pub fn new() -> KeyObject {
        JsCast::unchecked_into(Object::new())
    }
}
impl PeerCertificate {
    pub fn new() -> PeerCertificate {
        JsCast::unchecked_into(Object::new())
    }
}
impl PxfObject {
    pub fn new() -> PxfObject {
        JsCast::unchecked_into(Object::new())
    }
}
impl SecureContext {
    pub fn new() -> SecureContext {
        JsCast::unchecked_into(Object::new())
    }
}
impl SecureContextOptions {
    pub fn new() -> SecureContextOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl SecurePair {
    pub fn new() -> SecurePair {
        JsCast::unchecked_into(Object::new())
    }
}
impl Server {
    pub fn new() -> Server {
        JsCast::unchecked_into(Object::new())
    }
}
impl TLSSocket {
    pub fn new() -> TLSSocket {
        JsCast::unchecked_into(Object::new())
    }
}
impl TLSSocketOptions {
    pub fn new() -> TLSSocketOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl TlsOptions {
    pub fn new() -> TlsOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<CommonConnectionOptions> for ConnectionOptions {
    fn as_ref(&self) -> &CommonConnectionOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<ConnectionOptions> for CommonConnectionOptions {
    fn from(child: ConnectionOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<SecureContextOptions> for ConnectionOptions {
    fn as_ref(&self) -> &SecureContextOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<ConnectionOptions> for SecureContextOptions {
    fn from(child: ConnectionOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<PeerCertificate> for DetailedPeerCertificate {
    fn as_ref(&self) -> &PeerCertificate {
        JsCast::unchecked_ref(self)
    }
}
impl From<DetailedPeerCertificate> for PeerCertificate {
    fn from(child: DetailedPeerCertificate) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<EventEmitter> for Server {
    fn as_ref(&self) -> &EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<Server> for EventEmitter {
    fn from(child: Server) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Duplex> for TLSSocket {
    fn as_ref(&self) -> &Duplex {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for Duplex {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<EventEmitter> for TLSSocket {
    fn as_ref(&self) -> &EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for EventEmitter {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Readable> for TLSSocket {
    fn as_ref(&self) -> &Readable {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for Readable {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<ReadableStream> for TLSSocket {
    fn as_ref(&self) -> &ReadableStream {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for ReadableStream {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Socket> for TLSSocket {
    fn as_ref(&self) -> &Socket {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for Socket {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Stream> for TLSSocket {
    fn as_ref(&self) -> &Stream {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for Stream {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Writable> for TLSSocket {
    fn as_ref(&self) -> &Writable {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for Writable {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<WritableStream> for TLSSocket {
    fn as_ref(&self) -> &WritableStream {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocket> for WritableStream {
    fn from(child: TLSSocket) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<CommonConnectionOptions> for TLSSocketOptions {
    fn as_ref(&self) -> &CommonConnectionOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocketOptions> for CommonConnectionOptions {
    fn from(child: TLSSocketOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<SecureContextOptions> for TLSSocketOptions {
    fn as_ref(&self) -> &SecureContextOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<TLSSocketOptions> for SecureContextOptions {
    fn from(child: TLSSocketOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<CommonConnectionOptions> for TlsOptions {
    fn as_ref(&self) -> &CommonConnectionOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<TlsOptions> for CommonConnectionOptions {
    fn from(child: TlsOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<SecureContextOptions> for TlsOptions {
    fn as_ref(&self) -> &SecureContextOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<TlsOptions> for SecureContextOptions {
    fn from(child: TlsOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
