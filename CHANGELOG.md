# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Release automation:
  - `scripts/release.sh` with dependency-safe publish order.
  - `Makefile` targets for dry-run and publish.
- Workspace policy metadata:
  - `rust-version = "1.74"` in each crate.
  - docs.rs metadata (`all-features`, `--cfg docsrs`) in each crate.
- New space crates:
  - `rand_hsv`
  - `rand_hwb`
  - `rand_oklab`
  - `rand_oklch`
  - `rand_lab`
  - `rand_lch`
- New conversion strategy crate:
  - `rand_color_convert` with RGB <-> HSL conversion helpers and `ToRgb`/`ToHsl` traits.
- Property-based tests (`proptest`) for range invariants and deterministic seeded generation in workspace crates.

### Changed
- `rand_color` expanded to re-export all space crates plus `convert` module.
- Workspace root documentation updated with crate map and automation workflow.
- Version bumps for next releases:
  - `rand_rgb` -> `0.2.2`
  - `rand_hsl` -> `0.1.1`
  - `rand_color` -> `0.2.0`

## [0.2.0] - 2026-02-12

### Added
- Deterministic seeded RNG APIs:
  - `random_color_with_rng(&mut R) -> RandomColor`
  - `random_color_in_with_rng(ColorRange, &mut R) -> Result<RandomColor, ColorError>`
- `Display` implementation for `RandomColor`.
- Optional `serde` feature for serializing/deserializing public types.
- GitHub Actions CI workflow running `fmt`, `clippy`, `test`, and `doc`.

### Changed
- Package version bumped to `0.2.0` for API cleanup.
- Documentation updated with seeded usage and new API examples.

### Removed
- Removed deprecated legacy methods:
  - `RandomColor::rand_color_struct(...)`
  - `RandomColor::rand_color_string(...)`

## [0.1.1] - 2026-02-12

### Added
- New ergonomic API:
  - `random_color() -> RandomColor`
  - `random_color_in(ColorRange) -> Result<RandomColor, ColorError>`
  - `RandomColor::to_rgba_string() -> String`
- `ColorRange` config type with defaults and validation.
- `ColorError` for invalid input range handling.
- Rustdoc examples for public API and README usage examples.
- New changelog file.

### Changed
- `RandomColor` channels (`red`, `green`, `blue`, `alpha`) are now public.
- Generation now uses a single internal path for both struct and string output.
- `Cargo.toml` metadata improved (`homepage`, `documentation`, valid categories).
- Test suite now asserts behavior (bounds, validation, formatting, compatibility) instead of printing.

### Deprecated
- `RandomColor::rand_color_struct(...)` in favor of `random_color_in(...)`.
- `RandomColor::rand_color_string(...)` in favor of `random_color_in(...).to_rgba_string()`.

## [0.1.0]

### Added
- Initial release of `rand_rgb`.
