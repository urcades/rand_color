# rand_hsv

Generate random `hsva(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_hsv
```

Optional serde support:

```bash
cargo add rand_hsv --features serde
```

## Quick start

```rust
use rand_hsv::random_hsv;

let color = random_hsv();
let formatted = color.to_hsva_string();

assert!((0.0..=360.0).contains(&color.hue));
assert!(formatted.starts_with("hsva("));
```

## Custom ranges

```rust
use rand_hsv::{random_hsv_in, HsvRange};

let range = HsvRange::new(100.0, 200.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8).unwrap();
let color = random_hsv_in(range).unwrap();

assert!((100.0..=200.0).contains(&color.hue));
assert!((0.2..=0.8).contains(&color.alpha));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_hsv::random_hsv_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_hsv_with_rng(&mut rng_a), random_hsv_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `hsva(...)` is a stable crate compatibility format, not a browser-CSS guarantee.
