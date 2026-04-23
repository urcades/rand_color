# rand_oklch

Generate random `oklch(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_oklch
```

Optional serde support:

```bash
cargo add rand_oklch --features serde
```

## Quick start

```rust
use rand_oklch::random_oklch;

let color = random_oklch();
let formatted = color.to_oklch_string();

assert!((0.0..=360.0).contains(&color.hue));
assert!(formatted.starts_with("oklch("));
```

## Custom ranges

```rust
use rand_oklch::{random_oklch_in, OklchRange};

let range = OklchRange::new(0.2, 0.8, 0.05, 0.25, 100.0, 200.0, 0.2, 0.8).unwrap();
let color = random_oklch_in(range).unwrap();

assert!((0.2..=0.8).contains(&color.lightness));
assert!((100.0..=200.0).contains(&color.hue));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_oklch::random_oklch_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_oklch_with_rng(&mut rng_a), random_oklch_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `to_oklch_string()` returns stable crate formatting, not a full CSS serialization engine.
