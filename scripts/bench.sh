#!/usr/bin/env bash

cargo build --release --features runtime-benchmarks
echo "template"
./target/release/node-fennel-protocol benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_template --extrinsic '*' --steps 20 --repeat 100 --raw --output ./
echo "trust"
./target/release/node-fennel-protocol benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_trust --extrinsic '*' --steps 20 --repeat 100 --raw --output ./
echo "fennel-identity"
./target/release/node-fennel-protocol benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_fennel_identity --extrinsic '*' --steps 20 --repeat 100 --raw --output ./
echo "signal"
./target/release/node-fennel-protocol benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_signal --extrinsic '*' --steps 20 --repeat 100 --raw --output ./
echo "keystore"
./target/release/node-fennel-protocol benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_keystore --extrinsic '*' --steps 20 --repeat 100 --raw --output ./