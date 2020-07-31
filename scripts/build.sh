#!/bin/bash
set -e
cargo clean
echo "compiling..."
cargo check --quiet --workspace --all-targets --all-features
echo "OK"
echo "check format..."
cargo fmt --all -- --check
echo "OK"
cargo clean
echo "check lints..."
cargo clippy --quiet --workspace --all-targets --all-features -- -D warnings
echo "OK"
cargo clean
echo "building..."
cargo build --quiet --release --workspace --all-targets --all-features
echo "OK"
echo "testing libraries..."
cargo test --quiet --release --workspace --lib --all-features
echo "OK"
echo "testing solutions..."
cargo test --quiet --release --workspace --test solution --all-features
echo "OK"
