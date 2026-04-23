# rand_hsl

Generate random `hsla(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_hsl
```

Optional serde support:

```bash
cargo add rand_hsl --features serde
```

## Quick start

```rust
use rand_hsl::random_hsl;

let color = random_hsl();
let css = color.to_hsla_string();

assert!(css.starts_with("hsla("));
```

## Custom ranges

```rust
use rand_hsl::{random_hsl_in, HslRange};

let range = HslRange::new(100.0, 200.0, 20.0, 80.0, 15.0, 60.0, 0.2, 0.8).unwrap();
let color = random_hsl_in(range).unwrap();

assert!((100.0..=200.0).contains(&color.hue));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_hsl::random_hsl_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_hsl_with_rng(&mut rng_a), random_hsl_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- `to_hsla_string()` returns stable crate formatting with one decimal place for hue/saturation/lightness and two for alpha.
