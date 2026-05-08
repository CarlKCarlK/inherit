use std::collections::BTreeSet;

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

    fn from_values(values: &[T]) -> Self {
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
// then `I` automatically "is a" `IntoIterableOfRangeSetRefs` and gets
// the `union` method with no per-type impl.
trait IntoIterableOfRangeSetRefs<'a, T: 'a>: IntoIterator<Item = &'a RangeSetBlaze<T>> + Sized {
    fn union(self) -> RangeSetBlaze<T>
    where
        T: Ord + Copy + 'a,
    {
        let mut result = RangeSetBlaze::new();
        for set in self {
            result = result.union(set);
        }
        result
    }
}

// Any type that can be turned into an iterator of &RangeSetBlaze<T>
// automatically is-a IntoIterableOfRangeSetRefs.
impl<'a, T: 'a, I> IntoIterableOfRangeSetRefs<'a, T> for I where I: IntoIterator<Item = &'a RangeSetBlaze<T>> {}

fn main() {
    let a = RangeSetBlaze::from_values(&[1, 2, 3]);
    let b = RangeSetBlaze::from_values(&[3, 4, 5]);
    let c = RangeSetBlaze::from_values(&[5, 7, 9]);

    println!("union (Vec refs): {:?}", vec![&a, &b, &c].union());
    println!("union (array refs): {:?}", [&a, &b, &c].union());
}
