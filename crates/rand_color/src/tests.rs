#[test]
fn rgb_module_re_exports_work() {
    let color = crate::rgb::random_color();
    assert!(color.to_rgba_string().starts_with("rgba("));
}

#[test]
fn hsl_module_re_exports_work() {
    let color = crate::hsl::random_hsl();
    assert!(color.to_hsla_string().starts_with("hsla("));
}

#[test]
fn hsv_module_re_exports_work() {
    let color = crate::hsv::random_hsv();
    assert!(color.to_hsva_string().starts_with("hsva("));
}

#[test]
fn hwb_module_re_exports_work() {
    let color = crate::hwb::random_hwb();
    assert!(color.to_hwba_string().starts_with("hwba("));
}

#[test]
fn oklab_module_re_exports_work() {
    let color = crate::oklab::random_oklab();
    assert!(color.to_oklab_string().starts_with("oklab("));
}

#[test]
fn oklch_module_re_exports_work() {
    let color = crate::oklch::random_oklch();
    assert!(color.to_oklch_string().starts_with("oklch("));
}

#[test]
fn lab_module_re_exports_work() {
    let color = crate::lab::random_lab();
    assert!(color.to_lab_string().starts_with("lab("));
}

#[test]
fn lch_module_re_exports_work() {
    let color = crate::lch::random_lch();
    assert!(color.to_lch_string().starts_with("lch("));
}

#[test]
fn convert_module_re_exports_work() {
    let rgb = crate::rgb::random_color();
    let hsl = crate::convert::rgb_to_hsl(rgb);
    assert!((0.0..=360.0).contains(&hsl.hue));
}
