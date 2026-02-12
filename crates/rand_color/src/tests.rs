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
