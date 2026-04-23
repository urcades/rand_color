# rand_color_convert

Conversion strategy crate for `rand_color` workspace.

Currently includes:

- RGB <-> HSL conversion helpers
- lightweight conversion traits (`ToRgb`, `ToHsl`)

## Install

```bash
cargo add rand_color_convert
```

## Quick start

```rust
use rand_color_convert::{rgb_to_hsl, ToRgb};
use rand_rgb::RandomColor;

let rgb = RandomColor {
    red: 255,
    green: 0,
    blue: 0,
    alpha: 1.0,
};

let hsl = rgb_to_hsl(rgb);
let round_trip = hsl.to_rgb();

assert_eq!(round_trip.red, 255);
assert_eq!(round_trip.alpha, 1.0);
```

## Caveats

- Conversion support is intentionally limited to RGB <-> HSL for now.
- HSL-to-RGB wraps hue, clamps saturation/lightness into valid percentage ranges, and preserves alpha.
- This crate is not a full color-management or gamut-mapping engine.
