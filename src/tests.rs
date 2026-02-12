use crate::{
    random_color, random_color_in, random_color_in_with_rng, random_color_with_rng, ColorError,
    ColorRange, RandomColor,
};
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
