# rand_color Workspace

This repository is now a Cargo workspace for random color generation across multiple color spaces.

## Crates

- `rand_rgb`: RGB/RGBA generation crate. Kept stable for existing users.
- `rand_hsl`: HSL/HSLA generation crate.
- `rand_color`: umbrella crate that re-exports space-specific crates in modules.

## Which crate should I use?

- Use `rand_rgb` if you only need RGB.
- Use `rand_hsl` if you only need HSL.
- Use `rand_color` if you want one entrypoint and expect to mix spaces over time.

## Quick start (`rand_color`)

```rust
let rgb = rand_color::rgb::random_color();
let hsl = rand_color::hsl::random_hsl();

assert!(rgb.to_rgba_string().starts_with("rgba("));
assert!(hsl.to_hsla_string().starts_with("hsla("));
```

## Workspace checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```
