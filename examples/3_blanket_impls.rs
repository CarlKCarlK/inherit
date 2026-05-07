use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct RangeSetBlaze<T> {
    values: BTreeSet<T>,
}

impl<T: Ord + Copy> RangeSetBlaze<T> {
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
// then `I` automatically "is a" `MultiwayRangeSetBlazeRef` and gets
// the `union` method with no per-type impl.
trait MultiwayRangeSetBlazeRef<'a, T: 'a>: IntoIterator<Item = &'a RangeSetBlaze<T>> + Sized {
    fn union(self) -> RangeSetBlaze<T>
    where
        T: Ord + Copy + 'a,
    {
        let mut it = self.into_iter();
        let mut acc = it
            .next()
            .cloned()
            .unwrap_or_else(|| RangeSetBlaze { values: BTreeSet::new() });

        for set in it {
            acc = acc.union(set);
        }
        acc
    }
}

impl<'a, T: 'a, I> MultiwayRangeSetBlazeRef<'a, T> for I where I: IntoIterator<Item = &'a RangeSetBlaze<T>> {}

fn main() {
    let a = RangeSetBlaze::from_values(&[1, 2, 3]);
    let b = RangeSetBlaze::from_values(&[3, 4, 5]);
    let c = RangeSetBlaze::from_values(&[5, 7, 9]);

    println!("union (Vec refs): {:?}", vec![&a, &b, &c].union());
    println!("union (array refs): {:?}", [&a, &b, &c].union());
}
