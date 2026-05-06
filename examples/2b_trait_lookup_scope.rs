mod api {
    pub trait Ping {
        fn ping(&self) -> &'static str;
    }

    pub struct Service;

    impl Ping for Service {
        fn ping(&self) -> &'static str {
            "pong"
        }
    }
}

use api::Ping;

fn main() {
    let s = api::Service;

    // Works because `Ping` is in scope.
    println!("{}", s.ping());
}
