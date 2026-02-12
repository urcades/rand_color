use crate::{random_oklch, random_oklch_with_rng, OklchColor, OklchRange};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn default_bounds_work() {
    let color = random_oklch();
    assert!((0.0..=1.0).contains(&color.lightness));
    assert!((0.0..=0.4).contains(&color.chroma));
    assert!((0.0..=360.0).contains(&color.hue));
}

#[test]
fn format_precision() {
    let color = OklchColor {
        lightness: 0.1234,
        chroma: 0.2222,
        hue: 123.456,
        alpha: 0.8888,
    };
    assert_eq!(color.to_oklch_string(), "oklch(0.123, 0.222, 123.5, 0.89)");
}

proptest! {
    #[test]
    fn prop_seeded_oklch_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_oklch_with_rng(&mut a), random_oklch_with_rng(&mut b));
    }

    #[test]
    fn prop_default_range_accepts_values(seed in any::<u64>()) {
        let mut rng = StdRng::seed_from_u64(seed);
        let color = crate::random_oklch_in_with_rng(OklchRange::default(), &mut rng).unwrap();
        prop_assert!((0.0..=1.0).contains(&color.lightness));
        prop_assert!((0.0..=0.4).contains(&color.chroma));
        prop_assert!((0.0..=360.0).contains(&color.hue));
    }
}
