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

    // Inherent method wins on plain dot-call.
    println!("{}", p.role());

    // Universal Function Call Syntax (UFCS) disambiguates which trait method to call.
    // If you avoid traits with the same method name, you usually do not need this.
    println!("{}", Pilot::role(&p));
    println!("{}", Wizard::role(&p));
}
