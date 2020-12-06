#!/bin/bash
set -e

cargo fmt
mdl -ag .
cargo clean
cargo clippy -- -D clippy::all
cargo build --release
cargo test --release
