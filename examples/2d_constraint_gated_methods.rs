// Mini version of device-envoy's OutputArray pattern.
//
// Base impl: methods available for any N.
// Specialized impl: extra method only for N = 8.
//
// Inheritance-like idea:
//
// Common variant:
// - Define a general impl for all T.
// - Add another impl/methods only when extra bounds hold (for example,
//   methods that only make sense when T: Display).
// - All OutputArray<N> values have the base API.
// - OutputArray<8> has one extra capability.

#[derive(Debug, Clone, Copy)]
struct OutputArray<const N: usize> {
    levels: [bool; N],
}

impl<const N: usize> OutputArray<N> {
    fn new() -> Self {
        Self { levels: [false; N] }
    }

    fn set_level_at_index(&mut self, index: usize, level: bool) {
        if let Some(slot) = self.levels.get_mut(index) {
            *slot = level;
        }
    }
}

// Hardcoded 8, matching the bit width expected by a u8 mask.
impl OutputArray<8> {
    fn set_from_bits(&mut self, mut bits: u8) {
        for slot in &mut self.levels {
            *slot = (bits & 1) == 1;
            bits >>= 1;
        }
    }
}

fn main() {
    let mut any = OutputArray::<4>::new();
    any.set_level_at_index(2, true);

    let mut eight = OutputArray::<8>::new();
    eight.set_from_bits(0b1011_0001);

    println!("N=4 levels: {:?}", any.levels);
    println!("N=8 levels: {:?}", eight.levels);
}
