use crate::{
    random_color, random_color_in, random_color_in_with_rng, random_color_with_rng, ColorError,
    ColorRange, RandomColor,
};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn random_color_defaults_are_in_bounds() {
    let color = random_color();

    assert!((0.0..=1.0).contains(&color.alpha));
    assert!(color.to_rgba_string().starts_with("rgba("));
}

#[test]
fn random_color_with_custom_range_is_in_bounds() {
    let range = ColorRange::new(100, 200, 100, 200, 33, 200, 0.2, 0.8).unwrap();
    let color = random_color_in(range).unwrap();

    assert!((100..=200).contains(&color.red));
    assert!((100..=200).contains(&color.green));
    assert!((33..=200).contains(&color.blue));
    assert!((0.2..=0.8).contains(&color.alpha));
}

#[test]
fn seeded_default_generation_is_deterministic() {
    let mut rng_a = StdRng::seed_from_u64(42);
    let mut rng_b = StdRng::seed_from_u64(42);

    let color_a = random_color_with_rng(&mut rng_a);
    let color_b = random_color_with_rng(&mut rng_b);

    assert_eq!(color_a, color_b);
}

#[test]
fn seeded_custom_generation_is_deterministic() {
    let mut rng_a = StdRng::seed_from_u64(7);
    let mut rng_b = StdRng::seed_from_u64(7);
    let range_a = ColorRange::new(100, 200, 100, 200, 33, 200, 0.2, 0.8).unwrap();
    let range_b = ColorRange::new(100, 200, 100, 200, 33, 200, 0.2, 0.8).unwrap();

    let color_a = random_color_in_with_rng(range_a, &mut rng_a).unwrap();
    let color_b = random_color_in_with_rng(range_b, &mut rng_b).unwrap();

    assert_eq!(color_a, color_b);
}

#[test]
fn invalid_channel_range_returns_error() {
    let result = ColorRange::new(200, 100, 100, 200, 33, 200, 0.0, 1.0);
    assert_eq!(result, Err(ColorError::InvalidChannelRange));
}

#[test]
fn invalid_alpha_range_returns_error() {
    let result = ColorRange::new(100, 200, 100, 200, 33, 200, 0.8, 0.2);
    assert_eq!(result, Err(ColorError::InvalidAlphaRange));
}

#[test]
fn out_of_bounds_alpha_returns_error() {
    let result = ColorRange::new(100, 200, 100, 200, 33, 200, -0.1, 0.5);
    assert_eq!(result, Err(ColorError::AlphaOutOfBounds));
}

#[test]
fn rgba_string_is_formatted_with_two_decimals() {
    let color = RandomColor {
        red: 10,
        green: 20,
        blue: 30,
        alpha: 0.1234,
    };
    assert_eq!(color.to_rgba_string(), "rgba(10, 20, 30, 0.12)");
}

#[test]
fn display_matches_rgba_format() {
    let color = RandomColor {
        red: 12,
        green: 34,
        blue: 56,
        alpha: 0.9876,
    };
    assert_eq!(color.to_string(), "rgba(12, 34, 56, 0.99)");
}

proptest! {
    #[test]
    fn prop_seeded_generation_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_color_with_rng(&mut a), random_color_with_rng(&mut b));
    }

    #[test]
    fn prop_custom_range_respected(
        seed in any::<u64>(),
        a_r in 0u8..=255,
        b_r in 0u8..=255,
        a_g in 0u8..=255,
        b_g in 0u8..=255,
        a_b in 0u8..=255,
        b_b in 0u8..=255,
        a_a in 0.0f32..1.0f32,
        b_a in 0.0f32..1.0f32,
    ) {
        let min_r = a_r.min(b_r);
        let max_r = a_r.max(b_r);
        let min_g = a_g.min(b_g);
        let max_g = a_g.max(b_g);
        let min_b = a_b.min(b_b);
        let max_b = a_b.max(b_b);
        let min_a = a_a.min(b_a);
        let max_a = a_a.max(b_a);

        let range = ColorRange::new(min_r, max_r, min_g, max_g, min_b, max_b, min_a, max_a).unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        let color = random_color_in_with_rng(range.clone(), &mut rng).unwrap();

        prop_assert!((range.red.start().to_owned()..=range.red.end().to_owned()).contains(&color.red));
        prop_assert!((range.green.start().to_owned()..=range.green.end().to_owned()).contains(&color.green));
        prop_assert!((range.blue.start().to_owned()..=range.blue.end().to_owned()).contains(&color.blue));
        prop_assert!((range.alpha.0..=range.alpha.1).contains(&color.alpha));
    }
}
