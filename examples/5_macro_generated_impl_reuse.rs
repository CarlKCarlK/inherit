use std::net::{Ipv4Addr, Ipv6Addr};

// Closer to range-set-blaze style: explicit impl blocks + macro reuse.
// Simplified surface: only add_one/min_value/max_value.
trait Integer: Copy + Ord {
    fn add_one(self) -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! impl_integer_ops_num {
    ($t:ty) => {
        fn add_one(self) -> Self {
            self + 1
        }

        fn min_value() -> Self {
            <$t>::MIN
        }

        fn max_value() -> Self {
            <$t>::MAX
        }
    };
}

macro_rules! impl_integer_ops_ipv4 {
    () => {
        fn add_one(self) -> Self {
            Ipv4Addr::from(u32::from(self).wrapping_add(1))
        }

        fn min_value() -> Self {
            Ipv4Addr::from(0u32)
        }

        fn max_value() -> Self {
            Ipv4Addr::from(u32::MAX)
        }
    };
}

macro_rules! impl_integer_ops_ipv6 {
    () => {
        fn add_one(self) -> Self {
            Ipv6Addr::from(u128::from(self).wrapping_add(1))
        }

        fn min_value() -> Self {
            Ipv6Addr::from(0u128)
        }

        fn max_value() -> Self {
            Ipv6Addr::from(u128::MAX)
        }
    };
}

macro_rules! impl_integer_ops_char {
    () => {
        fn add_one(self) -> Self {
            let mut num = u32::from(self) + 1;
            if num == 0xD800 {
                num = 0xE000;
            }
            char::from_u32(num).unwrap_or(char::MIN)
        }

        fn min_value() -> Self {
            char::MIN
        }

        fn max_value() -> Self {
            char::MAX
        }
    };
}

impl Integer for i8 { impl_integer_ops_num!(i8); }
impl Integer for u8 { impl_integer_ops_num!(u8); }
impl Integer for i16 { impl_integer_ops_num!(i16); }
impl Integer for u16 { impl_integer_ops_num!(u16); }
impl Integer for i32 { impl_integer_ops_num!(i32); }
impl Integer for u32 { impl_integer_ops_num!(u32); }
impl Integer for i64 { impl_integer_ops_num!(i64); }
impl Integer for u64 { impl_integer_ops_num!(u64); }
impl Integer for i128 { impl_integer_ops_num!(i128); }
impl Integer for u128 { impl_integer_ops_num!(u128); }
impl Integer for isize { impl_integer_ops_num!(isize); }
impl Integer for usize { impl_integer_ops_num!(usize); }
impl Integer for char { impl_integer_ops_char!(); }
impl Integer for Ipv4Addr { impl_integer_ops_ipv4!(); }
impl Integer for Ipv6Addr { impl_integer_ops_ipv6!(); }

// Could just use direct impls for char/Ipv4Addr/Ipv6Addr, but this shows how to reuse the same macro for multiple types.

// Can't use default impls for Integer
// because MIN and MAX aren't defined on IPs
// and +1 isn't defined on IPs or char.

// Can't use num_traits::PrimInt as a 'subclass'
// because Rust worries that 'char' etc. might
// be added to it later (coherence).


fn main() {
    let x: u8 = 255;
    println!("u8 add_one({x}) = {}", x.add_one());

    println!("char min={:?} max={:?}", char::min_value(), char::max_value());
    println!(
        "ipv4 min={:?} max={:?}",
        Ipv4Addr::min_value(),
        Ipv4Addr::max_value()
    );
}
