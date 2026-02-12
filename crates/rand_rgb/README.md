# rand_rgb

Generate random `rgba(...)` colors as structured values or formatted strings.

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

## Compatibility note

`rand_rgb` is maintained for RGB-focused users and compatibility.

If you plan to use multiple spaces, prefer `rand_color::rgb` from the `rand_color` umbrella crate.
