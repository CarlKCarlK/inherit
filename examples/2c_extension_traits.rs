trait WordTools {
    fn first_word(&self) -> &str;
}

impl WordTools for str {
    fn first_word(&self) -> &str {
        self.split_whitespace().next().unwrap_or("")
    }
}

fn main() {
    let text = "rust favors composition";
    println!("{}", text.first_word());
}
