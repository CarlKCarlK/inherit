use std::collections::BTreeSet;

// For this example, use u64 as our stand-in integer type.
type Integer = u64;

// Mock up RangeSetBlaze as a BTreeSet wrapper for demo purposes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeSetBlaze {
    values: BTreeSet<Integer>,
}

impl RangeSetBlaze {
    fn new() -> Self {
        Self {
            values: BTreeSet::new(),
        }
    }

    // Construct from a slice of Integer values.
    fn from_slice(values: &[Integer]) -> Self {
        Self {
            values: values.iter().copied().collect(),
        }
    }

    // Return a new set that is the union of two borrowed sets.
    fn union(&self, other: &Self) -> Self {
        Self {
            values: self.values.union(&other.values).copied().collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// A RangeSetCollection must be iterable over borrowed RangeSetBlaze values.
// In return, the trait gives it a union() method over those values.
trait RangeSetCollection<'a>: IntoIterator<Item = &'a RangeSetBlaze> {
    fn union(self) -> RangeSetBlaze
    where
        Self: Sized,
    {
        let mut result = RangeSetBlaze::new();
        for set in self {
            result = RangeSetBlaze::union(&result, set);
        }
        result
    }
}

// TECHNIQUE NAME: blanket implementation
//
// This is broader than a normal extension trait impl.
// Instead of adding union() to one type, we add it to every type
// that can be turned into an iterator over &RangeSetBlaze.
impl<'a, I> RangeSetCollection<'a> for I where I: IntoIterator<Item = &'a RangeSetBlaze> {}

fn main() {
    let a = RangeSetBlaze::from_slice(&[1, 2, 3]);
    let b = RangeSetBlaze::from_slice(&[3, 4, 5]);
    let c = RangeSetBlaze::from_slice(&[5, 7, 9]);

    let expected = RangeSetBlaze::from_slice(&[1, 2, 3, 4, 5, 7, 9]);
    // A vector of sets can be unioned together.
    assert_eq!(vec![&a, &b, &c].union(), expected);
    // An array of sets can be unioned together.
    assert_eq!([&a, &b, &c].union(), expected);
    // A filtered option can be unioned.
    assert_eq!(Some(&a).filter(|set| !set.is_empty()).union(), a);
}
