// Macro-generated method bodies, modeled on range-set-blaze's Integer style.
//
// Why this is inheritance-like:
// - A shared trait contract is reused across many concrete types.
// - A macro emits repeated impl blocks and method logic.

trait Integer: Copy + Ord {
    fn add_one(self) -> Self;
    fn sub_one(self) -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! impl_integer {
    ($t:ty) => {
        impl Integer for $t {
            fn add_one(self) -> Self {
                self + 1
            }

            fn sub_one(self) -> Self {
                self - 1
            }

            fn min_value() -> Self {
                <$t>::MIN
            }

            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

// One-line per type.
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(i32);

fn show_bounds<T: Integer + core::fmt::Debug>() {
    println!("min={:?}, max={:?}", T::min_value(), T::max_value());
}

fn main() {
    let x: u8 = 10;
    println!("u8: {} -> {} -> {}", x, x.add_one(), x.sub_one());
    show_bounds::<u8>();
    show_bounds::<u16>();
    show_bounds::<i32>();
}
