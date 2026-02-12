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

## Usage

```rust
let rgb = rand_color::rgb::random_color();
let hsl = rand_color::hsl::random_hsl();
let hsv = rand_color::hsv::random_hsv();

assert!(rgb.to_rgba_string().starts_with("rgba("));
assert!(hsl.to_hsla_string().starts_with("hsla("));
assert!(hsv.to_hsva_string().starts_with("hsva("));
```
