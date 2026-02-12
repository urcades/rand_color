use crate::{
    random_hsl, random_hsl_in, random_hsl_in_with_rng, random_hsl_with_rng, HslColor, HslError,
    HslRange,
};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn random_hsl_defaults_are_in_bounds() {
    let color = random_hsl();

    assert!((0.0..=360.0).contains(&color.hue));
    assert!((0.0..=100.0).contains(&color.saturation));
    assert!((0.0..=100.0).contains(&color.lightness));
    assert!((0.0..=1.0).contains(&color.alpha));
}

#[test]
fn random_hsl_with_custom_range_is_in_bounds() {
    let range = HslRange::new(100.0, 200.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8).unwrap();
    let color = random_hsl_in(range).unwrap();

    assert!((100.0..=200.0).contains(&color.hue));
    assert!((20.0..=80.0).contains(&color.saturation));
    assert!((15.0..=60.0).contains(&color.lightness));
    assert!((0.2..=0.8).contains(&color.alpha));
}

#[test]
fn seeded_default_generation_is_deterministic() {
    let mut rng_a = StdRng::seed_from_u64(42);
    let mut rng_b = StdRng::seed_from_u64(42);

    assert_eq!(
        random_hsl_with_rng(&mut rng_a),
        random_hsl_with_rng(&mut rng_b)
    );
}

#[test]
fn seeded_custom_generation_is_deterministic() {
    let mut rng_a = StdRng::seed_from_u64(7);
    let mut rng_b = StdRng::seed_from_u64(7);
    let range_a = HslRange::new(100.0, 200.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8).unwrap();
    let range_b = HslRange::new(100.0, 200.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8).unwrap();

    let color_a = random_hsl_in_with_rng(range_a, &mut rng_a).unwrap();
    let color_b = random_hsl_in_with_rng(range_b, &mut rng_b).unwrap();

    assert_eq!(color_a, color_b);
}

#[test]
fn invalid_hue_range_returns_error() {
    let result = HslRange::new(300.0, 100.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8);
    assert_eq!(result, Err(HslError::InvalidHueRange));
}

#[test]
fn invalid_saturation_range_returns_error() {
    let result = HslRange::new(100.0, 200.0, 90.0, 10.0, 15.0, 60.0, 0.2, 0.8);
    assert_eq!(result, Err(HslError::InvalidSaturationRange));
}

#[test]
fn out_of_bounds_component_returns_error() {
    let result = HslRange::new(100.0, 400.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8);
    assert_eq!(result, Err(HslError::ComponentOutOfBounds));
}

#[test]
fn non_finite_component_returns_error() {
    let result = HslRange::new(f32::NAN, 200.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8);
    assert_eq!(result, Err(HslError::NonFiniteValue));
}

#[test]
fn hsla_string_is_formatted_with_precision() {
    let color = HslColor {
        hue: 123.456,
        saturation: 45.678,
        lightness: 89.123,
        alpha: 0.3333,
    };
    assert_eq!(color.to_hsla_string(), "hsla(123.5, 45.7%, 89.1%, 0.33)");
}

proptest! {
    #[test]
    fn prop_hsl_in_range(
        seed in any::<u64>(),
        a_h in 0.0f32..360.0f32,
        b_h in 0.0f32..360.0f32,
        a_s in 0.0f32..100.0f32,
        b_s in 0.0f32..100.0f32,
        a_l in 0.0f32..100.0f32,
        b_l in 0.0f32..100.0f32,
        a_a in 0.0f32..1.0f32,
        b_a in 0.0f32..1.0f32,
    ) {
        let min_h = a_h.min(b_h);
        let max_h = a_h.max(b_h);
        let min_s = a_s.min(b_s);
        let max_s = a_s.max(b_s);
        let min_l = a_l.min(b_l);
        let max_l = a_l.max(b_l);
        let min_a = a_a.min(b_a);
        let max_a = a_a.max(b_a);

        let range = HslRange::new(min_h, max_h, min_s, max_s, min_l, max_l, min_a, max_a).unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        let color = random_hsl_in_with_rng(range.clone(), &mut rng).unwrap();

        prop_assert!((range.hue.0..=range.hue.1).contains(&color.hue));
        prop_assert!((range.saturation.0..=range.saturation.1).contains(&color.saturation));
        prop_assert!((range.lightness.0..=range.lightness.1).contains(&color.lightness));
        prop_assert!((range.alpha.0..=range.alpha.1).contains(&color.alpha));
    }

    #[test]
    fn prop_seeded_hsl_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_hsl_with_rng(&mut a), random_hsl_with_rng(&mut b));
    }
}
