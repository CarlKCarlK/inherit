use std::ops::RangeInclusive;

// Mini `range-set-blaze` Integer
trait Integer: Copy + Ord {
    fn min_value() -> Self;
    fn max_value() -> Self;

    // Default behavior inherited by implementors.
    // Any impl can override this method.

    /// Returns an exhausted (empty) range`.
    fn exhausted_range() -> RangeInclusive<Self> {
        debug_assert!(Self::min_value() < Self::max_value(), "Precondition");
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

    assert!(r1.is_empty());
    assert!(r2.is_empty());
}
