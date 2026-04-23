# rand_color

Unified random color generation across color spaces.

Supported modules:

- `rgb` via `rand_rgb`
- `hsl` via `rand_hsl`
- `hsv` via `rand_hsv`
- `hwb` via `rand_hwb`
- `oklab` via `rand_oklab`
- `oklch` via `rand_oklch`
- `lab` via `rand_lab`
- `lch` via `rand_lch`
- `convert` via `rand_color_convert`

## Install

```bash
cargo add rand_color
```

Optional serde support for all color modules:

```bash
cargo add rand_color --features serde
```

## Quick start

```rust
let rgb = rand_color::rgb::random_color();
let hsl = rand_color::hsl::random_hsl();
let hsv = rand_color::hsv::random_hsv();

assert!(rgb.to_rgba_string().starts_with("rgba("));
assert!(hsl.to_hsla_string().starts_with("hsla("));
assert!(hsv.to_hsva_string().starts_with("hsva("));
```

## Custom ranges and seeded RNGs

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

## Conversion

`convert` currently supports RGB <-> HSL only.

```rust
let rgb = rand_color::rgb::random_color();
let hsl = rand_color::convert::rgb_to_hsl(rgb);

assert!((0.0..=360.0).contains(&hsl.hue));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- String helpers are stable crate formatting. `hsva(...)` and `hwba(...)` are compatibility formats, not browser-CSS guarantees.
