use std::ops::RangeInclusive;

// Mini version of `range-set-blaze`'s Integer pattern.
trait Integer: Copy + Ord {
    fn min_value() -> Self;
    fn max_value() -> Self;

    /// Returns an exhausted range, which is a range that starts from the maximum value and ends at the minimum value.
    /// This results in an empty range.
    //
    // Default behavior inherited by implementors.
    // Any impl can override this method if it needs specialized behavior.
    fn exhausted_range() -> RangeInclusive<Self> {
        debug_assert!(Self::min_value() < Self::max_value(), "Precondition violated: min_value must be less than max_value");
        Self::max_value()..=Self::min_value()
    }
}

impl Integer for u8 {
    fn min_value() -> Self {
        u8::MIN
    }

    fn max_value() -> Self {
        u8::MAX
    }
}

impl Integer for i16 {
    fn min_value() -> Self {
        i16::MIN
    }

    fn max_value() -> Self {
        i16::MAX
    }
}

fn main() {
    let r1 = u8::exhausted_range();
    let r2 = i16::exhausted_range();

    println!("u8 exhausted range is empty: {}", r1.is_empty());
    println!("i16 exhausted range is empty: {}", r2.is_empty());
}
