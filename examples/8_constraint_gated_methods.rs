#[derive(Debug, Clone, Copy)]
struct OutputArray<const N: usize> {
    levels: [bool; N],
}

impl<const N: usize> OutputArray<N> {
    fn new() -> Self {
        Self { levels: [false; N] }
    }

    fn set_level_at_index(&mut self, index: usize, level: bool) {
        self.levels[index] = level;
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

// TECHNIQUE NAME: constraint-gated methods.
// - Put methods in a separate impl block.
// - The methods exist only when the impl's constraints are met.
// - Constraints can be const values, trait bounds, lifetimes, or combinations

fn main() {
    let mut any = OutputArray::<4>::new();
    any.set_level_at_index(2, true);

    let mut eight = OutputArray::<8>::new();
    eight.set_from_bits(0b1011_0001);

    assert_eq!(any.levels, [false, false, true, false]);
    assert_eq!(eight.levels, [true, false, false, false, true, true, false, true]);
}
