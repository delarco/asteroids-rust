#!/bin/bash

# Exit immediately if a command fails
set -e

echo "Starting Rust game build..."
cargo build --profile release --target wasm32-unknown-unknown

echo "Build succeeded! Running wasm-bindgen..."
wasm-bindgen --out-dir ./dist/ --target web target/wasm32-unknown-unknown/release/asteroids-rust.wasm

if command -v wasm-opt >/dev/null 2>&1; then
    echo "wasm-bindgen succeeded! Running wasm-opt..."
    wasm-opt -Oz -o asteroids-rust_bg.wasm asteroids-rust_bg.wasm
else
    echo "wasm-bindgen succeeded! wasm-opt is not installed or not in PATH. Skipping."
fi

echo "Build complete! Binary located at: ./dist"
