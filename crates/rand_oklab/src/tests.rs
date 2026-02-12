use crate::{random_oklab, random_oklab_with_rng, OklabColor, OklabRange};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn default_bounds_work() {
    let color = random_oklab();
    assert!((0.0..=1.0).contains(&color.lightness));
    assert!((-0.4..=0.4).contains(&color.a));
    assert!((-0.4..=0.4).contains(&color.b));
}

#[test]
fn format_precision() {
    let color = OklabColor {
        lightness: 0.1234,
        a: -0.2222,
        b: 0.3333,
        alpha: 0.8888,
    };
    assert_eq!(color.to_oklab_string(), "oklab(0.123, -0.222, 0.333, 0.89)");
}

proptest! {
    #[test]
    fn prop_seeded_oklab_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_oklab_with_rng(&mut a), random_oklab_with_rng(&mut b));
    }

    #[test]
    fn prop_default_range_accepts_values(seed in any::<u64>()) {
        let mut rng = StdRng::seed_from_u64(seed);
        let color = crate::random_oklab_in_with_rng(OklabRange::default(), &mut rng).unwrap();
        prop_assert!((0.0..=1.0).contains(&color.lightness));
        prop_assert!((-0.4..=0.4).contains(&color.a));
        prop_assert!((-0.4..=0.4).contains(&color.b));
    }
}
