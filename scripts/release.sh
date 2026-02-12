#!/usr/bin/env bash
set -euo pipefail

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=true
fi

CRATES=(
  rand_hsl
  rand_hsv
  rand_hwb
  rand_oklab
  rand_oklch
  rand_lab
  rand_lch
  rand_color_convert
  rand_rgb
  rand_color
)

for crate in "${CRATES[@]}"; do
  if [[ "$DRY_RUN" == "true" ]]; then
    echo ">>> cargo check -p ${crate} --all-features"
    cargo check -p "${crate}" --all-features
  else
    echo ">>> cargo publish -p ${crate}"
    cargo publish -p "${crate}" --allow-dirty
  fi
done
