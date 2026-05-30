// Alternative solution without macros.
// Uses local supertraits PrimInt and PrimIp to share logic,
// with explicit impls for all 15 types.

use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::Add;

trait Integer: Copy + Ord {
    fn add_one(self) -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
}

// A local supertrait for the 12 numeric primitive integer types.
// Can't use num_traits::PrimInt as a 'supertrait' for a blanket impl
// because Rust worries that 'char' etc. might be added to it later (coherence).
trait PrimInt: Copy + Ord + Add<Output = Self> {
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

impl PrimInt for i8    { const ONE: Self = 1; const MIN: Self = i8::MIN;    const MAX: Self = i8::MAX;    }
impl PrimInt for u8    { const ONE: Self = 1; const MIN: Self = u8::MIN;    const MAX: Self = u8::MAX;    }
impl PrimInt for i16   { const ONE: Self = 1; const MIN: Self = i16::MIN;   const MAX: Self = i16::MAX;   }
impl PrimInt for u16   { const ONE: Self = 1; const MIN: Self = u16::MIN;   const MAX: Self = u16::MAX;   }
impl PrimInt for i32   { const ONE: Self = 1; const MIN: Self = i32::MIN;   const MAX: Self = i32::MAX;   }
impl PrimInt for u32   { const ONE: Self = 1; const MIN: Self = u32::MIN;   const MAX: Self = u32::MAX;   }
impl PrimInt for i64   { const ONE: Self = 1; const MIN: Self = i64::MIN;   const MAX: Self = i64::MAX;   }
impl PrimInt for u64   { const ONE: Self = 1; const MIN: Self = u64::MIN;   const MAX: Self = u64::MAX;   }
impl PrimInt for i128  { const ONE: Self = 1; const MIN: Self = i128::MIN;  const MAX: Self = i128::MAX;  }
impl PrimInt for u128  { const ONE: Self = 1; const MIN: Self = u128::MIN;  const MAX: Self = u128::MAX;  }
impl PrimInt for isize { const ONE: Self = 1; const MIN: Self = isize::MIN; const MAX: Self = isize::MAX; }
impl PrimInt for usize { const ONE: Self = 1; const MIN: Self = usize::MIN; const MAX: Self = usize::MAX; }

// A local supertrait for the IP address types.
trait PrimIp: Copy + Ord {
    type Repr: PrimInt;
    fn to_repr(self) -> Self::Repr;
    fn from_repr(repr: Self::Repr) -> Self;
    fn ip_add_one(self) -> Self { Self::from_repr(self.to_repr() + <Self::Repr as PrimInt>::ONE) }
    fn ip_min() -> Self { Self::from_repr(<Self::Repr as PrimInt>::MIN) }
    fn ip_max() -> Self { Self::from_repr(<Self::Repr as PrimInt>::MAX) }
}

impl PrimIp for Ipv4Addr {
    type Repr = u32;
    fn to_repr(self) -> u32 { u32::from(self) }
    fn from_repr(repr: u32) -> Self { Ipv4Addr::from(repr) }
}

impl PrimIp for Ipv6Addr {
    type Repr = u128;
    fn to_repr(self) -> u128 { u128::from(self) }
    fn from_repr(repr: u128) -> Self { Ipv6Addr::from(repr) }
}

// A local supertrait for char.
trait PrimChar {
    fn char_add_one(self) -> Self;
    fn char_min() -> Self;
    fn char_max() -> Self;
}

impl PrimChar for char {
    fn char_add_one(self) -> char {
        let mut num = u32::from(self) + 1;
        if num == 0xD800 { num = 0xE000; }
        char::from_u32(num).expect("next char must be a valid Unicode scalar value")
    }
    fn char_min() -> char { char::MIN }
    fn char_max() -> char { char::MAX }
}

impl Integer for i8    { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for u8    { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for i16   { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for u16   { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for i32   { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for u32   { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for i64   { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for u64   { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for i128  { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for u128  { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for isize { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for usize { fn add_one(self) -> Self { self + <Self as PrimInt>::ONE } fn min_value() -> Self { <Self as PrimInt>::MIN } fn max_value() -> Self { <Self as PrimInt>::MAX } }
impl Integer for Ipv4Addr { fn add_one(self) -> Self { self.ip_add_one() } fn min_value() -> Self { Self::ip_min() } fn max_value() -> Self { Self::ip_max() } }
impl Integer for Ipv6Addr { fn add_one(self) -> Self { self.ip_add_one() } fn min_value() -> Self { Self::ip_min() } fn max_value() -> Self { Self::ip_max() } }
impl Integer for char     { fn add_one(self) -> Self { self.char_add_one() } fn min_value() -> Self { Self::char_min() } fn max_value() -> Self { Self::char_max() } }

// TECHNIQUE NAME: local supertraits with explicit impls.

fn main() {
    let x: u8 = 254;
    assert_eq!(x.add_one(), 255);
    assert_eq!('a'.add_one(), 'b');

    assert_eq!(char::min_value(), char::MIN);
    assert_eq!(char::max_value(), char::MAX);
    assert_eq!(Ipv4Addr::min_value(), Ipv4Addr::new(0, 0, 0, 0));
    assert_eq!(Ipv4Addr::max_value(), Ipv4Addr::new(255, 255, 255, 255));
}
