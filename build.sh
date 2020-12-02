#!/bin/bash
set -e

cargo fmt
mdl -ag .
cargo clean
cargo clippy --workspace --all-targets --all-features
cargo build --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
