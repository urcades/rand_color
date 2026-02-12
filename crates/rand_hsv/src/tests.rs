use crate::{
    random_hsv, random_hsv_in_with_rng, random_hsv_with_rng, HsvColor, HsvError, HsvRange,
};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn default_bounds_work() {
    let color = random_hsv();
    assert!((0.0..=360.0).contains(&color.hue));
    assert!((0.0..=100.0).contains(&color.saturation));
    assert!((0.0..=100.0).contains(&color.value));
    assert!((0.0..=1.0).contains(&color.alpha));
}

#[test]
fn deterministic_seeded_generation() {
    let mut a = StdRng::seed_from_u64(1);
    let mut b = StdRng::seed_from_u64(1);
    assert_eq!(random_hsv_with_rng(&mut a), random_hsv_with_rng(&mut b));
}

#[test]
fn invalid_range_errors() {
    assert_eq!(
        HsvRange::new(10.0, 0.0, 0.0, 100.0, 0.0, 100.0, 0.0, 1.0),
        Err(HsvError::InvalidHueRange)
    );
}

#[test]
fn string_format_precision() {
    let color = HsvColor {
        hue: 12.345,
        saturation: 67.891,
        value: 45.678,
        alpha: 0.3333,
    };
    assert_eq!(color.to_hsva_string(), "hsva(12.3, 67.9%, 45.7%, 0.33)");
}

proptest! {
    #[test]
    fn prop_hsv_in_range(
        seed in any::<u64>(),
        a_h in 0.0f32..360.0f32,
        b_h in 0.0f32..360.0f32,
        a_s in 0.0f32..100.0f32,
        b_s in 0.0f32..100.0f32,
        a_v in 0.0f32..100.0f32,
        b_v in 0.0f32..100.0f32,
        a_a in 0.0f32..1.0f32,
        b_a in 0.0f32..1.0f32,
    ) {
        let min_h = a_h.min(b_h);
        let max_h = a_h.max(b_h);
        let min_s = a_s.min(b_s);
        let max_s = a_s.max(b_s);
        let min_v = a_v.min(b_v);
        let max_v = a_v.max(b_v);
        let min_a = a_a.min(b_a);
        let max_a = a_a.max(b_a);

        let range = HsvRange::new(min_h, max_h, min_s, max_s, min_v, max_v, min_a, max_a).unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        let color = random_hsv_in_with_rng(range.clone(), &mut rng).unwrap();

        prop_assert!((range.hue.0..=range.hue.1).contains(&color.hue));
        prop_assert!((range.saturation.0..=range.saturation.1).contains(&color.saturation));
        prop_assert!((range.value.0..=range.value.1).contains(&color.value));
        prop_assert!((range.alpha.0..=range.alpha.1).contains(&color.alpha));
    }

    #[test]
    fn prop_seeded_hsv_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_hsv_with_rng(&mut a), random_hsv_with_rng(&mut b));
    }
}
