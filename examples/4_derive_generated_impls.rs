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
