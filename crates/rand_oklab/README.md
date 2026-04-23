# rand_oklab

Generate random `oklab(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_oklab
```

Optional serde support:

```bash
cargo add rand_oklab --features serde
```

## Quick start

```rust
use rand_oklab::random_oklab;

let color = random_oklab();
let formatted = color.to_oklab_string();

assert!((0.0..=1.0).contains(&color.lightness));
assert!(formatted.starts_with("oklab("));
```

## Custom ranges

```rust
use rand_oklab::{random_oklab_in, OklabRange};

let range = OklabRange::new(0.2, 0.8, -0.2, 0.2, -0.2, 0.2, 0.2, 0.8).unwrap();
let color = random_oklab_in(range).unwrap();

assert!((0.2..=0.8).contains(&color.lightness));
assert!((0.2..=0.8).contains(&color.alpha));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_oklab::random_oklab_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_oklab_with_rng(&mut rng_a), random_oklab_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `to_oklab_string()` returns stable crate formatting, not a full CSS serialization engine.
