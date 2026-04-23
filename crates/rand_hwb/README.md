# rand_hwb

Generate random `hwba(...)` colors with configurable bounds.

## Install

```bash
cargo add rand_hwb
```

Optional serde support:

```bash
cargo add rand_hwb --features serde
```

## Quick start

```rust
use rand_hwb::random_hwb;

let color = random_hwb();
let formatted = color.to_hwba_string();

assert!((0.0..=360.0).contains(&color.hue));
assert!(color.whiteness + color.blackness <= 100.0);
assert!(formatted.starts_with("hwba("));
```

## Custom ranges

```rust
use rand_hwb::{random_hwb_in, HwbRange};

let range = HwbRange::new(100.0, 200.0, 20.0, 40.0, 10.0, 30.0, 0.2, 0.8).unwrap();
let color = random_hwb_in(range).unwrap();

assert!((100.0..=200.0).contains(&color.hue));
assert!((0.2..=0.8).contains(&color.alpha));
```

## Seeded deterministic generation

```rust
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_hwb::random_hwb_with_rng;

let mut rng_a = StdRng::seed_from_u64(42);
let mut rng_b = StdRng::seed_from_u64(42);

assert_eq!(random_hwb_with_rng(&mut rng_a), random_hwb_with_rng(&mut rng_b));
```

## Caveats

- Values are sampled from numeric component ranges; they are not gamut-checked, contrast-checked, palette-aware, or perceptually uniform.
- Generated whiteness and blackness are normalized if their sum exceeds `100.0`.
- `hwba(...)` is a stable crate compatibility format, not a browser-CSS guarantee.
