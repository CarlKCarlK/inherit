trait Draw {
    fn draw(&self);
}

struct Button;
struct Slider;

impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}

impl Draw for Slider {
    fn draw(&self) {
        println!("draw slider");
    }
}

fn main() {
    let ui: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Slider)];

    for w in ui {
        w.draw();
    }
}
