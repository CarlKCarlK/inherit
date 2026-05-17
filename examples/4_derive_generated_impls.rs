// Mini version of device-envoy's LedLevel derive pattern.
// This is not class inheritance; derive generates trait implementations for us.
//
// "is-a" framing:
// - LedLevel is-a Default trait
// - LedLevel is-a Debug trait
// - LedLevel is-a Clone/Copy traits
// - LedLevel is-a Eq/Ord/Hash traits
//
// How to think about it:
// - 1a (default methods): reuse comes from code inside a trait definition.
// - 1d (derive): reuse comes from macro-generated impl blocks.
//
// So this is behavior reuse through traits, but generated instead of handwritten.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default)]
enum LedLevel {
    On,
    #[default]
    Off,
}

fn main() {
    let default_level = LedLevel::default();
    let on = LedLevel::On;
    let off = LedLevel::Off;

    assert_eq!(default_level, LedLevel::Off);
    assert_ne!(on, off);
    assert!(off > on);

    // `Copy` + `Clone` come from derive too.
    let copied = on;
    let cloned = off.clone();
    assert_eq!(copied, on);
    assert_eq!(cloned, off);
}
