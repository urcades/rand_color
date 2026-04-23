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
  rand_rgb
  rand_color_convert
  rand_color
)

if [[ "$DRY_RUN" != "true" && -n "$(git status --short)" ]]; then
  echo "error: working tree must be clean before publishing" >&2
  git status --short >&2
  exit 1
fi

for crate in "${CRATES[@]}"; do
  if [[ "$DRY_RUN" == "true" ]]; then
    echo ">>> cargo check -p ${crate} --all-features"
    cargo check -p "${crate}" --all-features
  else
    echo ">>> cargo publish -p ${crate}"
    cargo publish -p "${crate}"
  fi
done
