// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl CpuInfo {
    pub fn new() -> CpuInfo {
        JsCast::unchecked_into(Object::new())
    }
}
impl NetworkInterfaceBase {
    pub fn new() -> NetworkInterfaceBase {
        JsCast::unchecked_into(Object::new())
    }
}
impl NetworkInterfaceInfoIPv4 {
    pub fn new() -> NetworkInterfaceInfoIPv4 {
        JsCast::unchecked_into(Object::new())
    }
}
impl NetworkInterfaceInfoIPv6 {
    pub fn new() -> NetworkInterfaceInfoIPv6 {
        JsCast::unchecked_into(Object::new())
    }
}
impl UserInfo {
    pub fn new() -> UserInfo {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<NetworkInterfaceBase> for NetworkInterfaceInfoIPv4 {
    fn as_ref(&self) -> &NetworkInterfaceBase {
        JsCast::unchecked_ref(self)
    }
}
impl From<NetworkInterfaceInfoIPv4> for NetworkInterfaceBase {
    fn from(child: NetworkInterfaceInfoIPv4) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<NetworkInterfaceBase> for NetworkInterfaceInfoIPv6 {
    fn as_ref(&self) -> &NetworkInterfaceBase {
        JsCast::unchecked_ref(self)
    }
}
impl From<NetworkInterfaceInfoIPv6> for NetworkInterfaceBase {
    fn from(child: NetworkInterfaceInfoIPv6) -> Self {
        JsCast::unchecked_into(child)
    }
}