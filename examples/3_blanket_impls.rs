use std::collections::BTreeSet;

// Mock up RangeSetBlaze as a BTreeSet wrapper for demo purposes.
#[derive(Debug, Clone)]
struct RangeSetBlaze<T> {
    values: BTreeSet<T>,
}

impl<T: Ord + Copy> RangeSetBlaze<T> {
    fn new() -> Self {
        Self {
            values: BTreeSet::new(),
        }
    }

    fn from_slice(values: &[T]) -> Self {
        Self {
            values: values.iter().copied().collect(),
        }
    }

    fn union(&self, other: &Self) -> Self {
        let mut out = self.values.clone();
        out.extend(other.values.iter().copied());
        Self { values: out }
    }
}

// Borrowed blanket-impl variant: any iterator of &RangeSetBlaze<T>
// automatically gains `union` without consuming sets.
//
// Faux-inheritance / "is-a" angle:
// If a type `I` satisfies `IntoIterator<Item = &RangeSetBlaze<T>>`,
// then `I` automatically "is a" `RangeSetRefIterable` and gets
// the `union` method with no per-type impl.
trait RangeSetRefIterable<T>: Sized {
    fn union<'a>(self) -> RangeSetBlaze<T>
    where
        T: Ord + Copy + 'a,
        Self: IntoIterator<Item = &'a RangeSetBlaze<T>>,
    {
        let mut result = RangeSetBlaze::new();
        for set in self {
            result = RangeSetBlaze::union(&result, set);
        }
        result
    }
}

// Any type can opt into this extension trait; the method itself is gated by
// the IntoIterator<Item = &RangeSetBlaze<T>> requirement above.
impl<T, I> RangeSetRefIterable<T> for I {}

fn main() {
    let a = RangeSetBlaze::from_slice(&[1, 2, 3]);
    let b = RangeSetBlaze::from_slice(&[3, 4, 5]);
    let c = RangeSetBlaze::from_slice(&[5, 7, 9]);

    let expected = RangeSetBlaze::from_slice(&[1, 2, 3, 4, 5, 7, 9]);
    assert_eq!(vec![&a, &b, &c].union().values, expected.values);
    assert_eq!([&a, &b, &c].union().values, expected.values);
}
