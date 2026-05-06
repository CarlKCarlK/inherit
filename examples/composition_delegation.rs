struct Engine {
    hp: u32,
}

impl Engine {
    fn describe(&self) -> String {
        format!("{} hp", self.hp)
    }
}

struct Car {
    engine: Engine,
}

impl Car {
    fn engine_description(&self) -> String {
        self.engine.describe()
    }
}

fn main() {
    let car = Car {
        engine: Engine { hp: 180 },
    };

    println!("Car engine: {}", car.engine_description());
}
