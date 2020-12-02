#!/bin/bash
set -e

cargo fmt
cargo check --workspace --all-targets --all-features
cargo clean
cargo clippy --workspace --all-targets --all-features
cargo build --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
