use std::ops::Deref;

struct Name(String);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let n = Name("turing".to_string());

    // `to_uppercase` is a `str` method, found via deref lookup.
    println!("{}", n.to_uppercase());
}
