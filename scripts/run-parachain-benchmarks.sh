#!/bin/bash

# Run Benchmarks for Parachain FRAME pallets

steps=$1
repeat=$2


pallets=(
    pallet_balances
    pallet_timestamp
    pallet_session
)

for p in ${pallets[@]}
do
  ./target/release/fennel-node benchmark pallet \
    --chain=fennel-local \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet=$p \
    --extrinsic='*' \
    --steps=$steps \
    --repeat=$repeat \
    --template=./scripts/templates/parachain-weight-template.hbs \
    --output=./runtime/fennel/src/weights
done
