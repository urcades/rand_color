use crate::{hsl_to_rgb, rgb_to_hsl, ToHsl, ToRgb};
use proptest::prelude::*;
use rand_hsl::HslColor;
use rand_rgb::RandomColor;

#[test]
fn known_rgb_to_hsl_red() {
    let rgb = RandomColor {
        red: 255,
        green: 0,
        blue: 0,
        alpha: 1.0,
    };
    let hsl = rgb_to_hsl(rgb);
    assert!((hsl.hue - 0.0).abs() < 0.01);
    assert!((hsl.saturation - 100.0).abs() < 0.01);
    assert!((hsl.lightness - 50.0).abs() < 0.01);
}

#[test]
fn known_hsl_to_rgb_red() {
    let hsl = HslColor {
        hue: 0.0,
        saturation: 100.0,
        lightness: 50.0,
        alpha: 1.0,
    };
    let rgb = hsl_to_rgb(hsl);
    assert_eq!(rgb.red, 255);
    assert_eq!(rgb.green, 0);
    assert_eq!(rgb.blue, 0);
}

#[test]
fn trait_calls_work() {
    let rgb = RandomColor {
        red: 128,
        green: 64,
        blue: 32,
        alpha: 0.75,
    };
    let hsl = rgb.to_hsl();
    let back = hsl.to_rgb();
    assert_eq!(back.alpha, 0.75);
}

proptest! {
    #[test]
    fn prop_round_trip_rgb(seed_r in any::<u8>(), seed_g in any::<u8>(), seed_b in any::<u8>()) {
        let rgb = RandomColor {
            red: seed_r,
            green: seed_g,
            blue: seed_b,
            alpha: 1.0,
        };
        let hsl = rgb_to_hsl(rgb);
        let round_trip = hsl_to_rgb(hsl);

        prop_assert!((round_trip.red as i16 - rgb.red as i16).abs() <= 1);
        prop_assert!((round_trip.green as i16 - rgb.green as i16).abs() <= 1);
        prop_assert!((round_trip.blue as i16 - rgb.blue as i16).abs() <= 1);
    }
}
