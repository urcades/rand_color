# Release Runbook

This workspace publishes crates in dependency order.

## Preconditions

1. Working tree is clean.
2. CI is green.
3. `cargo fmt`, `cargo clippy`, `cargo test`, and `RUSTDOCFLAGS="-D warnings" cargo doc` pass locally.
4. `CARGO_REGISTRY_TOKEN` is set for crates.io.

## Quick commands

Dry run:

```bash
make release-dry-run
```

Publish:

```bash
make release-publish
```

`release-publish` refuses to publish from a dirty working tree. `release-dry-run`
can be used during development before committing release-prep changes.

## Manual publish order

If publishing manually, use this order:

1. `rand_hsl`
2. `rand_hsv`
3. `rand_hwb`
4. `rand_oklab`
5. `rand_oklch`
6. `rand_lab`
7. `rand_lch`
8. `rand_rgb`
9. `rand_color_convert`
10. `rand_color`

## Common failure handling

- `429 Too Many Requests` from crates.io:
  - Wait until the timestamp in the error response and retry remaining crates.
- Missing dependency version on crates.io:
  - Publish the required dependency crate first, then retry.

## Post-release checklist

1. Tag released versions.
2. Push tags.
3. Verify versions on crates.io.
4. Announce release notes (if applicable).
