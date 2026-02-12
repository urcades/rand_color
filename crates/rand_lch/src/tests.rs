use crate::{random_lch, random_lch_with_rng, LchColor};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn default_bounds_work() {
    let color = random_lch();
    assert!((0.0..=100.0).contains(&color.lightness));
    assert!((0.0..=150.0).contains(&color.chroma));
    assert!((0.0..=360.0).contains(&color.hue));
}

#[test]
fn format_precision() {
    let color = LchColor {
        lightness: 12.345,
        chroma: 67.891,
        hue: 45.678,
        alpha: 0.3333,
    };
    assert_eq!(color.to_lch_string(), "lch(12.35, 67.89, 45.7, 0.33)");
}

proptest! {
    #[test]
    fn prop_seeded_lch_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_lch_with_rng(&mut a), random_lch_with_rng(&mut b));
    }
}
