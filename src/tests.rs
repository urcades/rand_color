use crate::{random_color, random_color_in, ColorError, ColorRange, RandomColor};

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
#[allow(deprecated)]
fn deprecated_struct_api_still_works() {
    let color = RandomColor::rand_color_struct(100, 200, 100, 200, 33, 200, 0.0, 1.0);

    assert!((100..=200).contains(&color.red));
    assert!((100..=200).contains(&color.green));
    assert!((33..=200).contains(&color.blue));
    assert!((0.0..=1.0).contains(&color.alpha));
}

#[test]
#[allow(deprecated)]
fn deprecated_string_api_still_works() {
    let color = RandomColor::rand_color_string(100, 200, 100, 200, 33, 200, 0.0, 1.0);
    assert!(color.starts_with("rgba("));
}
