# rand_lch

Generate random `lch(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_lch
```

Optional serde support:

```bash
cargo add rand_lch --features serde
```

## Quick start

```rust
use rand_lch::random_lch;

let color = random_lch();
let formatted = color.to_lch_string();

assert!((0.0..=360.0).contains(&color.hue));
assert!(formatted.starts_with("lch("));
```

## Custom ranges

```rust
use rand_lch::{random_lch_in, LchRange};

let range = LchRange::new(20.0, 80.0, 10.0, 70.0, 100.0, 200.0, 0.2, 0.8).unwrap();
let color = random_lch_in(range).unwrap();

assert!((20.0..=80.0).contains(&color.lightness));
assert!((100.0..=200.0).contains(&color.hue));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_lch::random_lch_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_lch_with_rng(&mut rng_a), random_lch_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `to_lch_string()` returns stable crate formatting, not a full CSS serialization engine.
