#!/usr/bin/env bash

rustup update nightly
rustup update stable
rustup override set nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

cargo build
cargo test
cargo doc

cargo build --release
cargo doc --release
