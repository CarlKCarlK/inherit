// Alternative solution without macros.
// Each type explicitly implements Integer and selects a shared family via an
// associated type. No blanket impls of Integer, so no coherence overlap.

use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::Add;

trait IntegerOps<T> {
    fn add_one(value: T) -> T;
    fn min_value() -> T;
    fn max_value() -> T;
}

trait Integer: Copy + Ord {
    type Ops: IntegerOps<Self>;

    fn add_one(self) -> Self   { Self::Ops::add_one(self) }
    fn min_value() -> Self     { Self::Ops::min_value() }
    fn max_value() -> Self     { Self::Ops::max_value() }
}

// ── PrimIntOps ────────────────────────────────────────────────────────────────

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

struct PrimIntOps;

impl<T: PrimInt> IntegerOps<T> for PrimIntOps {
    fn add_one(value: T) -> T  { value + T::ONE }
    fn min_value() -> T        { T::MIN }
    fn max_value() -> T        { T::MAX }
}

// ── PrimIpOps ─────────────────────────────────────────────────────────────────

trait PrimIp: Copy + Ord {
    type Repr: PrimInt;
    fn to_repr(self) -> Self::Repr;
    fn from_repr(repr: Self::Repr) -> Self;
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

struct PrimIpOps;

impl<T: PrimIp> IntegerOps<T> for PrimIpOps {
    fn add_one(value: T) -> T  { T::from_repr(value.to_repr() + T::Repr::ONE) }
    fn min_value() -> T        { T::from_repr(T::Repr::MIN) }
    fn max_value() -> T        { T::from_repr(T::Repr::MAX) }
}

// ── CharOps ───────────────────────────────────────────────────────────────────

struct CharOps;

impl IntegerOps<char> for CharOps {
    fn add_one(value: char) -> char {
        let mut num = u32::from(value) + 1;
        if num == 0xD800 { num = 0xE000; }
        char::from_u32(num).expect("next char must be a valid Unicode scalar value")
    }
    fn min_value() -> char { char::MIN }
    fn max_value() -> char { char::MAX }
}

// ── Integer impls: one per type, just selects the Ops family ──────────────────

impl Integer for i8    { type Ops = PrimIntOps; }
impl Integer for u8    { type Ops = PrimIntOps; }
impl Integer for i16   { type Ops = PrimIntOps; }
impl Integer for u16   { type Ops = PrimIntOps; }
impl Integer for i32   { type Ops = PrimIntOps; }
impl Integer for u32   { type Ops = PrimIntOps; }
impl Integer for i64   { type Ops = PrimIntOps; }
impl Integer for u64   { type Ops = PrimIntOps; }
impl Integer for i128  { type Ops = PrimIntOps; }
impl Integer for u128  { type Ops = PrimIntOps; }
impl Integer for isize { type Ops = PrimIntOps; }
impl Integer for usize { type Ops = PrimIntOps; }
impl Integer for Ipv4Addr { type Ops = PrimIpOps; }
impl Integer for Ipv6Addr { type Ops = PrimIpOps; }
impl Integer for char     { type Ops = CharOps; }

// TECHNIQUE NAME: associated-type ops selection.

fn main() {
    let x: u8 = 254;
    assert_eq!(x.add_one(), 255);
    assert_eq!('a'.add_one(), 'b');

    assert_eq!(char::min_value(), char::MIN);
    assert_eq!(char::max_value(), char::MAX);
    assert_eq!(Ipv4Addr::min_value(), Ipv4Addr::new(0, 0, 0, 0));
    assert_eq!(Ipv4Addr::max_value(), Ipv4Addr::new(255, 255, 255, 255));
}

