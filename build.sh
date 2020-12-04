#!/bin/bash
set -e

cargo fmt
mdl -ag .
cargo clean
cargo clippy
cargo build
cargo test
