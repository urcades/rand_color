use crate::{
    random_hwb, random_hwb_in_with_rng, random_hwb_with_rng, HwbColor, HwbError, HwbRange,
};
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn default_bounds_work() {
    let color = random_hwb();
    assert!((0.0..=360.0).contains(&color.hue));
    assert!((0.0..=100.0).contains(&color.whiteness));
    assert!((0.0..=100.0).contains(&color.blackness));
    assert!((0.0..=1.0).contains(&color.alpha));
    assert!(color.whiteness + color.blackness <= 100.0);
}

#[test]
fn seeded_generation_is_deterministic() {
    let mut a = StdRng::seed_from_u64(5);
    let mut b = StdRng::seed_from_u64(5);
    assert_eq!(random_hwb_with_rng(&mut a), random_hwb_with_rng(&mut b));
}

#[test]
fn string_format_precision() {
    let color = HwbColor {
        hue: 12.345,
        whiteness: 67.891,
        blackness: 15.111,
        alpha: 0.3333,
    };
    assert_eq!(color.to_hwba_string(), "hwba(12.3, 67.9%, 15.1%, 0.33)");
}

#[test]
fn invalid_white_black_combo_errors() {
    assert_eq!(
        HwbRange::new(0.0, 360.0, 70.0, 80.0, 40.0, 50.0, 0.0, 1.0),
        Err(HwbError::InvalidWhitenessBlacknessCombination)
    );
}

proptest! {
    #[test]
    fn prop_seeded_hwb_is_deterministic(seed in any::<u64>()) {
        let mut a = StdRng::seed_from_u64(seed);
        let mut b = StdRng::seed_from_u64(seed);
        prop_assert_eq!(random_hwb_with_rng(&mut a), random_hwb_with_rng(&mut b));
    }

    #[test]
    fn prop_hwb_in_range(seed in any::<u64>(), hue in 0.0f32..360.0f32, white in 0.0f32..100.0f32, black in 0.0f32..100.0f32, alpha in 0.0f32..1.0f32) {
        prop_assume!(white + black <= 100.0);
        let range = HwbRange::new(hue, hue, white, white, black, black, alpha, alpha).unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        let color = random_hwb_in_with_rng(range, &mut rng).unwrap();
        prop_assert_eq!(color.hue, hue);
        prop_assert!(color.whiteness >= 0.0);
        prop_assert!(color.blackness >= 0.0);
        prop_assert!(color.whiteness + color.blackness <= 100.0);
        prop_assert_eq!(color.alpha, alpha);
    }
}
