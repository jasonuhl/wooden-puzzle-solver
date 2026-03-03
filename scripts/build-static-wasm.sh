#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
RUST_CRATE_DIR="$ROOT_DIR/rust/wooden-puzzle-solver"
OUT_DIR="$ROOT_DIR/site/wasm"
WASM_BINDGEN_BIN="${WASM_BINDGEN_BIN:-$HOME/.cargo/bin/wasm-bindgen}"

mkdir -p "$OUT_DIR"
rm -rf "$OUT_DIR/debug" "$OUT_DIR/release"

cargo build --target wasm32-unknown-unknown --manifest-path "$RUST_CRATE_DIR/Cargo.toml"

cargo build --target wasm32-unknown-unknown --release --manifest-path "$RUST_CRATE_DIR/Cargo.toml"

"$WASM_BINDGEN_BIN" \
  --target web \
  --out-dir "$OUT_DIR" \
  "$RUST_CRATE_DIR/target/wasm32-unknown-unknown/release/wooden_puzzle_solver.wasm"

echo "Built wasm targets: debug + release"
echo "Installed release web assets in: $OUT_DIR"
