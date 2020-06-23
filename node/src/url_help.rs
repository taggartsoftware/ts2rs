// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl URL {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl URLFormatOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl URLSearchParams {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl Url {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl UrlObject {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl UrlWithParsedQuery {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl UrlWithStringQuery {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<Iterable> for URLSearchParams {
    fn as_ref(&self) -> &Iterable {
        self.unchecked_ref()
    }
}
impl From<URLSearchParams> for Iterable {
    fn from(child: URLSearchParams) -> Self {
        child.unchecked_into()
    }
}
impl URLSearchParams {
    pub fn to_iterable(self) -> Iterable {
        self.unchecked_into()
    }
    pub fn as_iterable(&self) -> &Iterable {
        self.unchecked_ref()
    }
}
impl AsRef<Url> for UrlWithParsedQuery {
    fn as_ref(&self) -> &Url {
        self.unchecked_ref()
    }
}
impl From<UrlWithParsedQuery> for Url {
    fn from(child: UrlWithParsedQuery) -> Self {
        child.unchecked_into()
    }
}
impl UrlWithParsedQuery {
    pub fn to_url(self) -> Url {
        self.unchecked_into()
    }
    pub fn as_url(&self) -> &Url {
        self.unchecked_ref()
    }
}
impl AsRef<Url> for UrlWithStringQuery {
    fn as_ref(&self) -> &Url {
        self.unchecked_ref()
    }
}
impl From<UrlWithStringQuery> for Url {
    fn from(child: UrlWithStringQuery) -> Self {
        child.unchecked_into()
    }
}
impl UrlWithStringQuery {
    pub fn to_url(self) -> Url {
        self.unchecked_into()
    }
    pub fn as_url(&self) -> &Url {
        self.unchecked_ref()
    }
}
