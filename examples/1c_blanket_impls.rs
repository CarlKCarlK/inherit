trait Speak {
    fn speak(&self) -> String;
}

trait SpeakTwice {
    fn speak_twice(&self) -> String;
}

impl<T: Speak> SpeakTwice for T {
    fn speak_twice(&self) -> String {
        format!("{} | {}", self.speak(), self.speak())
    }
}

struct Bird;

impl Speak for Bird {
    fn speak(&self) -> String {
        "tweet".to_string()
    }
}

fn main() {
    println!("{}", Bird.speak_twice());
}
