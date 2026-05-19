// struct HtmlBuffer;
// impl String for HtmlBuffer {
// }
// Error because String is not a trait.

use std::ops::{Deref, DerefMut};

// HtmlBuffer 'wraps' a String
struct HtmlBuffer(String);

impl HtmlBuffer {
    fn new() -> Self {
        Self(String::new())
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

// TECHNIQUE NAME: deref lookup.

fn main() {
    let mut page = HtmlBuffer::new();

    // `push_str` and `len` are String methods found via deref lookup.
    page.push_str("<h1>Hello</h1>");
    page.push_str("<p>Rust</p>");

    assert_eq!(page.len(), 25);
    // Borrow the thing `page` points to and compare.
    assert_eq!(&*page, "<h1>Hello</h1><p>Rust</p>");
}

// Good for `Box`, `Rc`, etc., but not really good here.
// Also see AsRef, From, and Borrow traits.