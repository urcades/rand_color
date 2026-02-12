# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
