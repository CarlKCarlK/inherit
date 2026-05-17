// 2b: Extension trait, modeled closely on range-set-blaze's UsizeExtensions.
//
// "is-a" framing:
// - usize is-a UsizeExtensions-able thing (it implements UsizeExtensions).
// - So usize values can call `.is_odd()` via trait method lookup.
//
// Relationship to 1a:
// - This is basically 1a (trait method reuse), but on OPT: other people's types.
// - We do not own `usize`, but we can add methods through our own trait.

trait UsizeExtensions {
    fn is_odd(self) -> bool;
}

impl UsizeExtensions for usize {
    fn is_odd(self) -> bool {
        self & 1 != 0
    }
}

fn main() {
    let count: usize = 7;
    let size: usize = 12;

    assert!(count.is_odd());
    assert!(!size.is_odd());
}
