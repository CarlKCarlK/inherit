trait Pilot {
    fn role(&self) -> &'static str {
        "pilot"
    }
}

trait Wizard {
    fn role(&self) -> &'static str {
        "wizard"
    }
}

struct Person;

impl Pilot for Person {}
impl Wizard for Person {}

impl Person {
    fn role(&self) -> &'static str {
        "person"
    }
}

fn main() {
    let p = Person;

    println!("{}", p.role());
    println!("{}", Pilot::role(&p));
    println!("{}", Wizard::role(&p));
}
