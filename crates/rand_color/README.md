# rand_color

Unified random color generation across color spaces.

Currently supports:

- `rgb` via `rand_rgb`
- `hsl` via `rand_hsl`

## Install

```bash
cargo add rand_color
```

Optional serde support for all re-exported crates:

```bash
cargo add rand_color --features serde
```

## Usage

```rust
let rgb = rand_color::rgb::random_color();
let hsl = rand_color::hsl::random_hsl();

assert!(rgb.to_rgba_string().starts_with("rgba("));
assert!(hsl.to_hsla_string().starts_with("hsla("));
```

## Migration from `rand_rgb`

`rand_rgb` remains available and maintained. For multi-space projects, prefer importing via `rand_color::rgb` so adding other spaces later is straightforward.
