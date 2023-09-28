#!/usr/bin/env bash
set -e

# build smart contracts
cargo build --package without-nep145 --target wasm32-unknown-unknown --release
cargo build --package with-nep145 --target wasm32-unknown-unknown --release

# run test
cargo run --package test-crate
