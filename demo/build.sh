#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="wasm32-unknown-unknown"
WASM_BINDGEN_VERSION="0.2.118"
TOOLS_DIR="${ROOT}/demo/.tools"
TARGET_DIR="${ROOT}/demo/target"
OUT_DIR="${ROOT}/demo/site/pkg"

if ! rustup target list --installed | grep -qx "${TARGET}"; then
  rustup target add "${TARGET}"
fi

mkdir -p "${TOOLS_DIR}"
export PATH="${TOOLS_DIR}/bin:${PATH}"

if ! command -v wasm-bindgen >/dev/null || ! wasm-bindgen --version | grep -q "${WASM_BINDGEN_VERSION}"; then
  cargo install wasm-bindgen-cli \
    --version "${WASM_BINDGEN_VERSION}" \
    --locked \
    --root "${TOOLS_DIR}"
fi

cargo build \
  --manifest-path "${ROOT}/demo/wasm/Cargo.toml" \
  --target-dir "${TARGET_DIR}" \
  --target "${TARGET}" \
  --release

rm -rf "${OUT_DIR}"
wasm-bindgen \
  --target web \
  --out-dir "${OUT_DIR}" \
  "${TARGET_DIR}/${TARGET}/release/rand_color_demo_wasm.wasm"

echo "Built demo WASM bindings in ${OUT_DIR}"
