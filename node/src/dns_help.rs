// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl AnyARecord {
    pub fn new() -> AnyARecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyAaaaRecord {
    pub fn new() -> AnyAaaaRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyCnameRecord {
    pub fn new() -> AnyCnameRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyMxRecord {
    pub fn new() -> AnyMxRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyNaptrRecord {
    pub fn new() -> AnyNaptrRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyNsRecord {
    pub fn new() -> AnyNsRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyPtrRecord {
    pub fn new() -> AnyPtrRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnySoaRecord {
    pub fn new() -> AnySoaRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnySrvRecord {
    pub fn new() -> AnySrvRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AnyTxtRecord {
    pub fn new() -> AnyTxtRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl LookupAddress {
    pub fn new() -> LookupAddress {
        JsCast::unchecked_into(Object::new())
    }
}
impl LookupAllOptions {
    pub fn new() -> LookupAllOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl LookupOneOptions {
    pub fn new() -> LookupOneOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl LookupOptions {
    pub fn new() -> LookupOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl MxRecord {
    pub fn new() -> MxRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl NaptrRecord {
    pub fn new() -> NaptrRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl RecordWithTtl {
    pub fn new() -> RecordWithTtl {
        JsCast::unchecked_into(Object::new())
    }
}
impl ResolveOptions {
    pub fn new() -> ResolveOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl ResolveWithTtlOptions {
    pub fn new() -> ResolveWithTtlOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl Resolver {
    pub fn new() -> Resolver {
        JsCast::unchecked_into(Object::new())
    }
}
impl SoaRecord {
    pub fn new() -> SoaRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl SrvRecord {
    pub fn new() -> SrvRecord {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<RecordWithTtl> for AnyARecord {
    fn as_ref(&self) -> &RecordWithTtl {
        JsCast::unchecked_ref(self)
    }
}
impl From<AnyARecord> for RecordWithTtl {
    fn from(child: AnyARecord) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<RecordWithTtl> for AnyAaaaRecord {
    fn as_ref(&self) -> &RecordWithTtl {
        JsCast::unchecked_ref(self)
    }
}
impl From<AnyAaaaRecord> for RecordWithTtl {
    fn from(child: AnyAaaaRecord) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<MxRecord> for AnyMxRecord {
    fn as_ref(&self) -> &MxRecord {
        JsCast::unchecked_ref(self)
    }
}
impl From<AnyMxRecord> for MxRecord {
    fn from(child: AnyMxRecord) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<NaptrRecord> for AnyNaptrRecord {
    fn as_ref(&self) -> &NaptrRecord {
        JsCast::unchecked_ref(self)
    }
}
impl From<AnyNaptrRecord> for NaptrRecord {
    fn from(child: AnyNaptrRecord) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<SoaRecord> for AnySoaRecord {
    fn as_ref(&self) -> &SoaRecord {
        JsCast::unchecked_ref(self)
    }
}
impl From<AnySoaRecord> for SoaRecord {
    fn from(child: AnySoaRecord) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<SrvRecord> for AnySrvRecord {
    fn as_ref(&self) -> &SrvRecord {
        JsCast::unchecked_ref(self)
    }
}
impl From<AnySrvRecord> for SrvRecord {
    fn from(child: AnySrvRecord) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<LookupOptions> for LookupAllOptions {
    fn as_ref(&self) -> &LookupOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<LookupAllOptions> for LookupOptions {
    fn from(child: LookupAllOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<LookupOptions> for LookupOneOptions {
    fn as_ref(&self) -> &LookupOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<LookupOneOptions> for LookupOptions {
    fn from(child: LookupOneOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<ResolveOptions> for ResolveWithTtlOptions {
    fn as_ref(&self) -> &ResolveOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<ResolveWithTtlOptions> for ResolveOptions {
    fn from(child: ResolveWithTtlOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}