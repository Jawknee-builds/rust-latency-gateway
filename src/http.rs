use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)] pub enum Method { Get, Post }
#[derive(Clone, Debug)] pub struct Request { pub method: Method, pub path: String }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct StatusCode(pub u16);
impl StatusCode { pub const OK: Self = Self(200); pub const NOT_FOUND: Self = Self(404); }
#[derive(Clone, Debug)] pub struct Response { pub status: StatusCode, pub body: String, pub headers: BTreeMap<String, String> }
impl Response {
    pub fn new(status: StatusCode, body: impl Into<String>) -> Self { Self { status, body: body.into(), headers: BTreeMap::new() } }
    pub fn ok(body: impl Into<String>) -> Self { Self::new(StatusCode::OK, body) }
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.headers.insert(key.into(), value.into()); self }
}
