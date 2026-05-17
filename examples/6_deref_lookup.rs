use std::borrow::Borrow;
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
#[derive(Debug, Clone)]
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
    // Use when you want wrapper ergonomics that mostly feel like the inner type.
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

// Use From for infallible owned conversions (may still have real runtime cost).
impl From<String> for HtmlBuffer {
    fn from(value: String) -> Self {
        Self(value)
    }
}

// Use AsRef for cheap explicit borrowed-view conversion in API boundaries.
impl AsRef<str> for HtmlBuffer {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Use Borrow when borrowed and owned forms must have matching Eq/Ord/Hash semantics.
impl Borrow<str> for HtmlBuffer {
    fn borrow(&self) -> &str {
        &self.0
    }
}

fn text_len(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

fn borrowed_text(s: impl Borrow<str>) -> String {
    let text: &str = s.borrow();
    text.to_string()
}

fn main() {
    let mut page = HtmlBuffer::new();

    // `push_str` and `len` are String methods found via deref lookup.
    page.push_str("<h1>Hello</h1>");
    page.push_str("<p>Rust</p>");

    assert_eq!(page.len(), page.as_bytes().len());
    assert_eq!(&*page, "<h1>Hello</h1><p>Rust</p>");

    let html2: HtmlBuffer = "<h2>World</h2>".to_string().into(); // From<String>
    assert_eq!(text_len(&html2), "<h2>World</h2>".len()); // AsRef<str>
    assert_eq!(borrowed_text(html2), "<h2>World</h2>"); // Borrow<str>
}
