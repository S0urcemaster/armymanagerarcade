#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "$0")/.." && pwd)"
output_dir="$project_dir/dist"

mkdir -p "$output_dir"
cargo build --release --target wasm32-unknown-unknown
cp "$project_dir/target/wasm32-unknown-unknown/release/army-manager-arcade.wasm" "$output_dir/"
cp "$project_dir/web/index.html" "$output_dir/"
cp "$project_dir/web/storage.js" "$output_dir/"

echo "Built web client in $output_dir"
