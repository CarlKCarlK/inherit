trait Greeter {
    fn name(&self) -> &'static str;

    fn greet(&self) {
        println!("Hello from {}", self.name());
    }
}

struct Robot;

impl Greeter for Robot {
    fn name(&self) -> &'static str {
        "Robot"
    }
}

fn main() {
    Robot.greet();
}
