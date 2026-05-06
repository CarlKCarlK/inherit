trait Named {
    fn name(&self) -> &'static str;
}

trait LoudNamed: Named {
    fn shout_name(&self) {
        println!("{}!", self.name().to_uppercase());
    }
}

struct Cat;

impl Named for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }
}

impl LoudNamed for Cat {}

fn main() {
    Cat.shout_name();
}
