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

## Install

```bash
cargo add rand_color
```

Optional serde support for all generated color structs:

```bash
cargo add rand_color --features serde
```

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

## Custom ranges and seeded RNGs

Every focused generation crate exposes:

- a color value type, such as `rand_rgb::RandomColor`
- a range type, such as `rand_rgb::ColorRange`
- `random_*()` for thread-local randomness
- `random_*_in(...)` for custom bounds
- `random_*_with_rng(...)` and `random_*_in_with_rng(...)` for deterministic tests

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_color::rgb::{random_color_in_with_rng, ColorRange};

let range = ColorRange::new(100, 200, 50, 150, 0, 80, 0.4, 1.0).unwrap();
let mut rng = StdRng::seed_from_u64(42);
let color = random_color_in_with_rng(range, &mut rng).unwrap();

assert!((100..=200).contains(&color.red));
assert!((0.4..=1.0).contains(&color.alpha));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `rand_color_convert` currently provides RGB <-> HSL conversion only.
- String helpers are stable crate formatting. `rgba(...)`, `hsla(...)`, `oklab(...)`, `oklch(...)`, `lab(...)`, and `lch(...)` mirror common color notation; `hsva(...)` and `hwba(...)` are compatibility formats, not browser-CSS guarantees.

## Local quality checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```

## Release automation

```bash
make release-dry-run
make release-publish
```

The publish script enforces dependency-safe release order.

For security reporting and patch process details, see `SECURITY.md`.
For the release checklist and publish order, see `RELEASE.md`.
