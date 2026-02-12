# rand_color Workspace

This repository is a Cargo workspace for random color generation across multiple color spaces.

## Crates

- `rand_rgb`: RGB/RGBA generation crate (compatibility-focused for existing users).
- `rand_hsl`: HSL/HSLA generation crate.
- `rand_hsv`: HSV/HSVA generation crate.
- `rand_hwb`: HWB generation crate.
- `rand_oklab`: Oklab generation crate.
- `rand_oklch`: Oklch generation crate.
- `rand_lab`: CIELAB generation crate.
- `rand_lch`: CIELCH generation crate.
- `rand_color_convert`: conversion strategy crate (currently RGB <-> HSL helpers + traits).
- `rand_color`: umbrella crate that re-exports all spaces and conversion helpers.

## Which crate should I use?

- Use a focused crate (`rand_rgb`, `rand_hsl`, etc.) for single-space use.
- Use `rand_color` for a unified entrypoint across spaces.
- Use `rand_color_convert` directly if you only need conversion helpers.

## Maintenance mode

This workspace is now feature-frozen and maintained in bugfix/security mode.

- New color spaces or major API additions are out of scope unless there is a clear correctness or compatibility need.
- Existing APIs and crate boundaries are intended to remain stable.
- Contributions are welcome for bug fixes, docs fixes, CI/release reliability, and security updates.

## Quick start (`rand_color`)

```rust
let rgb = rand_color::rgb::random_color();
let hsl = rand_color::hsl::random_hsl();
let oklab = rand_color::oklab::random_oklab();

let converted = rand_color::convert::rgb_to_hsl(rgb);

assert!(rgb.to_rgba_string().starts_with("rgba("));
assert!(hsl.to_hsla_string().starts_with("hsla("));
assert!(oklab.to_oklab_string().starts_with("oklab("));
assert!((0.0..=360.0).contains(&converted.hue));
```

## Local quality checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

## Release automation

```bash
make release-dry-run
make release-publish
```

The publish script enforces dependency-safe release order.

For security reporting and patch process details, see `SECURITY.md`.
For the release checklist and publish order, see `RELEASE.md`.
