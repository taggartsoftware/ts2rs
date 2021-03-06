// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl Agent {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AgentOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Server {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<crate::http::Agent> for Agent {
    fn as_ref(&self) -> &crate::http::Agent {
        self.unchecked_ref()
    }
}
impl From<Agent> for crate::http::Agent {
    fn from(child: Agent) -> Self {
        child.unchecked_into()
    }
}
impl Agent {
    pub fn to_http_agent(self) -> crate::http::Agent {
        self.unchecked_into()
    }
    pub fn as_http_agent(&self) -> &crate::http::Agent {
        self.unchecked_ref()
    }
}
impl AsRef<crate::http::AgentOptions> for AgentOptions {
    fn as_ref(&self) -> &crate::http::AgentOptions {
        self.unchecked_ref()
    }
}
impl From<AgentOptions> for crate::http::AgentOptions {
    fn from(child: AgentOptions) -> Self {
        child.unchecked_into()
    }
}
impl AgentOptions {
    pub fn to_http_agent_options(self) -> crate::http::AgentOptions {
        self.unchecked_into()
    }
    pub fn as_http_agent_options(&self) -> &crate::http::AgentOptions {
        self.unchecked_ref()
    }
}
impl AsRef<crate::tls::CommonConnectionOptions> for AgentOptions {
    fn as_ref(&self) -> &crate::tls::CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl From<AgentOptions> for crate::tls::CommonConnectionOptions {
    fn from(child: AgentOptions) -> Self {
        child.unchecked_into()
    }
}
impl AgentOptions {
    pub fn to_tls_common_connection_options(self) -> crate::tls::CommonConnectionOptions {
        self.unchecked_into()
    }
    pub fn as_tls_common_connection_options(&self) -> &crate::tls::CommonConnectionOptions {
        self.unchecked_ref()
    }
}
impl AsRef<crate::tls::ConnectionOptions> for AgentOptions {
    fn as_ref(&self) -> &crate::tls::ConnectionOptions {
        self.unchecked_ref()
    }
}
impl From<AgentOptions> for crate::tls::ConnectionOptions {
    fn from(child: AgentOptions) -> Self {
        child.unchecked_into()
    }
}
impl AgentOptions {
    pub fn to_tls_connection_options(self) -> crate::tls::ConnectionOptions {
        self.unchecked_into()
    }
    pub fn as_tls_connection_options(&self) -> &crate::tls::ConnectionOptions {
        self.unchecked_ref()
    }
}
impl AsRef<crate::tls::SecureContextOptions> for AgentOptions {
    fn as_ref(&self) -> &crate::tls::SecureContextOptions {
        self.unchecked_ref()
    }
}
impl From<AgentOptions> for crate::tls::SecureContextOptions {
    fn from(child: AgentOptions) -> Self {
        child.unchecked_into()
    }
}
impl AgentOptions {
    pub fn to_tls_secure_context_options(self) -> crate::tls::SecureContextOptions {
        self.unchecked_into()
    }
    pub fn as_tls_secure_context_options(&self) -> &crate::tls::SecureContextOptions {
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
impl AsRef<net::Server> for Server {
    fn as_ref(&self) -> &net::Server {
        self.unchecked_ref()
    }
}
impl From<Server> for net::Server {
    fn from(child: Server) -> Self {
        child.unchecked_into()
    }
}
impl Server {
    pub fn to_net_server(self) -> net::Server {
        self.unchecked_into()
    }
    pub fn as_net_server(&self) -> &net::Server {
        self.unchecked_ref()
    }
}
impl AsRef<crate::tls::Server> for Server {
    fn as_ref(&self) -> &crate::tls::Server {
        self.unchecked_ref()
    }
}
impl From<Server> for crate::tls::Server {
    fn from(child: Server) -> Self {
        child.unchecked_into()
    }
}
impl Server {
    pub fn to_tls_server(self) -> crate::tls::Server {
        self.unchecked_into()
    }
    pub fn as_tls_server(&self) -> &crate::tls::Server {
        self.unchecked_ref()
    }
}
