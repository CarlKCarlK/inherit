use std::net::{Ipv4Addr, Ipv6Addr};

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

macro_rules! impl_integer_ops_ip {
    ($ip_type:ty, $representation_type:ty) => {
        fn add_one(self) -> Self {
            <$ip_type>::from(<$representation_type>::from(self) + 1)
        }

        fn min_value() -> Self {
            <$ip_type>::from(<$representation_type>::MIN)
        }

        fn max_value() -> Self {
            <$ip_type>::from(<$representation_type>::MAX)
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
            char::from_u32(num).expect("next char must be a valid Unicode scalar value")
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
impl Integer for Ipv4Addr { impl_integer_ops_ip!(Ipv4Addr, u32); }
impl Integer for Ipv6Addr { impl_integer_ops_ip!(Ipv6Addr, u128); }
impl Integer for char { impl_integer_ops_char!(); }

// Can't use num_traits::PrimInt as a 'subclass'
// because Rust worries that 'char' etc. might
// be added to it later (coherence).
// HOWEVER: You could define your own PrimInt with 12 impls.

// TECHNIQUE NAME: macro-generated implementation.

fn main() {
    let x: u8 = 254;
    assert_eq!(x.add_one(), 255);
    assert_eq!('a'.add_one(), 'b');

    assert_eq!(char::min_value(), char::MIN);
    assert_eq!(char::max_value(), char::MAX);
    assert_eq!(Ipv4Addr::min_value(), Ipv4Addr::new(0, 0, 0, 0));
    assert_eq!(Ipv4Addr::max_value(), Ipv4Addr::new(255, 255, 255, 255));
}
