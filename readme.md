# rand_rgb

Generate random `rgba(...)` colors as either structured values or formatted strings.

## About

Inspired by "[random-rgb-color](https://github.com/mrmrs/random-rgb-color/tree/main)" by [mrmrs](https://mrmrs.cc/), which was in turn inspired by [random-hex-color](http://github.com/johno/random-hex-color) by [John Otander](https://www.johno.com/).

## Install

```bash
cargo add rand_rgb
```

## Quick start

```rust
use rand_rgb::random_color;

let color = random_color();
let css = color.to_rgba_string();

assert!(css.starts_with("rgba("));
```

## Custom ranges

```rust
use rand_rgb::{random_color_in, ColorRange};

let range = ColorRange::new(100, 200, 100, 200, 33, 200, 0.2, 0.8).unwrap();
let color = random_color_in(range).unwrap();

assert!((100..=200).contains(&color.red));
assert!((0.2..=0.8).contains(&color.alpha));
```

## API overview

- `random_color() -> RandomColor`: generates a color using defaults (`0..=255` for channels, `0.0..=1.0` for alpha).
- `random_color_in(ColorRange) -> Result<RandomColor, ColorError>`: generates a color with custom bounds and validation.
- `RandomColor::to_rgba_string() -> String`: formats with two decimal places in alpha.

## Compatibility note

`RandomColor::rand_color_struct(...)` and `RandomColor::rand_color_string(...)` still exist for compatibility but are deprecated in favor of the newer API.

## Next steps

- [ ] Add seeded RNG helpers for deterministic output in tests and demos.
- [ ] Consider implementing `Display` for `RandomColor`.
- [ ] Add benches if performance tuning becomes a goal.
