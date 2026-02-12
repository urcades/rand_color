use crate::{random_lab, random_lab_with_rng, LabColor};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn default_bounds_work() {
    let color = random_lab();
    assert!((0.0..=100.0).contains(&color.lightness));
    assert!((-128.0..=127.0).contains(&color.a));
    assert!((-128.0..=127.0).contains(&color.b));
}

#[test]
fn format_precision() {
    let color = LabColor {
        lightness: 12.345,
        a: -67.891,
        b: 45.678,
        alpha: 0.3333,
    };
    assert_eq!(color.to_lab_string(), "lab(12.35, -67.89, 45.68, 0.33)");
}

proptest! {
    #[test]
    fn prop_seeded_lab_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_lab_with_rng(&mut a), random_lab_with_rng(&mut b));
    }
}
