# rand_lab

Generate random `lab(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_lab
```

Optional serde support:

```bash
cargo add rand_lab --features serde
```

## Quick start

```rust
use rand_lab::random_lab;

let color = random_lab();
let formatted = color.to_lab_string();

assert!((0.0..=100.0).contains(&color.lightness));
assert!(formatted.starts_with("lab("));
```

## Custom ranges

```rust
use rand_lab::{random_lab_in, LabRange};

let range = LabRange::new(20.0, 80.0, -40.0, 40.0, -30.0, 30.0, 0.2, 0.8).unwrap();
let color = random_lab_in(range).unwrap();

assert!((20.0..=80.0).contains(&color.lightness));
assert!((0.2..=0.8).contains(&color.alpha));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_lab::random_lab_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_lab_with_rng(&mut rng_a), random_lab_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `to_lab_string()` returns stable crate formatting, not a full CSS serialization engine.
