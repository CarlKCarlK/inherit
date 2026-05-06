use std::ops::{Deref, DerefMut};

// Mini version of device-envoy's HtmlBuffer pattern:
// wrapper type that derefs to an inner String.
//
// Why this feels inheritance-like:
// - You can call many String methods on HtmlBuffer via deref lookup.
// - It feels like inheriting methods from a concrete type.
//
// But it is not subtype inheritance:
// - HtmlBuffer is not a String.
// - Method calls are resolved by automatic deref coercion.
//
// Coercion limits:
// - &HtmlBuffer can coerce to &String and then &str.
// - HtmlBuffer does NOT automatically become owned String.
//
// Design tradeoff:
// - Great ergonomics for a newtype wrapper.
// - But weaker encapsulation: callers can use lots of String API.
//   If you want strict newtype boundaries, avoid Deref and expose only explicit methods.
#[repr(transparent)]
struct HtmlBuffer(String);

impl HtmlBuffer {
    fn new() -> Self {
        Self(String::new())
    }

    // Note: String already has as_bytes(); this wrapper method is explicit API.
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Deref for HtmlBuffer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HtmlBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut page = HtmlBuffer::new();

    // `push_str` and `len` are String methods found via deref lookup.
    page.push_str("<h1>Hello</h1>");
    page.push_str("<p>Rust</p>");

    println!("html length: {}", page.len());
    println!("bytes length: {}", page.as_bytes().len());
    println!("html: {}", &*page);
}
