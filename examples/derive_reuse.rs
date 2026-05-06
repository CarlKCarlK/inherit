#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let a = User {
        id: 7,
        name: "Ada".to_string(),
    };
    let b = a.clone();

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("same value: {}", a == b);
}
