trait UsizeExtensions {
    fn is_odd(self) -> bool;
}

impl UsizeExtensions for usize {
    fn is_odd(self) -> bool {
        self & 1 != 0
    }
}

// TECHNIQUE NAME: extension traits.

fn main() {
    let count: usize = 7;

    assert!(count.is_odd());
    assert!(!12.is_odd());
}
