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
impl AsRef<crate::os::NetworkInterfaceBase> for crate::os::NetworkInterfaceInfoIPv4 {
    fn as_ref(&self) -> &crate::os::NetworkInterfaceBase {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::os::NetworkInterfaceInfoIPv4> for crate::os::NetworkInterfaceBase {
    fn from(child: crate::os::NetworkInterfaceInfoIPv4) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<crate::os::NetworkInterfaceBase> for crate::os::NetworkInterfaceInfoIPv6 {
    fn as_ref(&self) -> &crate::os::NetworkInterfaceBase {
        JsCast::unchecked_ref(self)
    }
}
impl From<crate::os::NetworkInterfaceInfoIPv6> for crate::os::NetworkInterfaceBase {
    fn from(child: crate::os::NetworkInterfaceInfoIPv6) -> Self {
        JsCast::unchecked_into(child)
    }
}
