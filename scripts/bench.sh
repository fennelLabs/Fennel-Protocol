#!/usr/bin/env bash

cargo build --release --features runtime-benchmarks

pallets=(
  "pallet_trust"
  "pallet_fennel_identity"
  "pallet_signal"
  "pallet_keystore"
)

for p in ${pallets[@]}
do
    ./target/release/fennel-node benchmark pallet \
        --chain fennel-local \
        --execution wasm \
        --wasm-execution compiled \
        --pallet $p \
        --extrinsic '*' \
        --steps 20 \
        --repeat 100 \
        --output ./
done
