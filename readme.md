# rand_rgb

Generate random `rgba(...)` colors as structured values or formatted strings.

## About

Inspired by "[random-rgb-color](https://github.com/mrmrs/random-rgb-color/tree/main)" by [mrmrs](https://mrmrs.cc/), which was in turn inspired by [random-hex-color](http://github.com/johno/random-hex-color) by [John Otander](https://www.johno.com/).

## Install

```bash
cargo add rand_rgb
```

Optional serde support:

```bash
cargo add rand_rgb --features serde
```

## Quick start

```rust
use rand_rgb::random_color;

let color = random_color();
assert!((0.0..=1.0).contains(&color.alpha));

// Display and to_rgba_string produce the same CSS-style format.
let css = color.to_string();
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

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_rgb::random_color_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(
    random_color_with_rng(&mut rng_a),
    random_color_with_rng(&mut rng_b)
);
```

## API overview

- `random_color() -> RandomColor`
- `random_color_with_rng(&mut R) -> RandomColor`
- `random_color_in(ColorRange) -> Result<RandomColor, ColorError>`
- `random_color_in_with_rng(ColorRange, &mut R) -> Result<RandomColor, ColorError>`
- `RandomColor::to_rgba_string() -> String`
- `Display` for `RandomColor` (`format!("{}", color)`)

## Breaking change in 0.2.0

Deprecated legacy methods were removed:

- `RandomColor::rand_color_struct(...)`
- `RandomColor::rand_color_string(...)`

Use `random_color_in(...)` plus `to_rgba_string()` instead.
