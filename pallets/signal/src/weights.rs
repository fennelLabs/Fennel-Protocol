//! Autogenerated weights for `pallet_signal`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-02, STEPS: `100`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_signal
// --extrinsic=*
// --steps=100
// --repeat=100
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_signal.
pub trait WeightInfo {
    fn send_rating_signal() -> Weight;
    fn update_rating_signal() -> Weight;
    fn revoke_rating_signal() -> Weight;
    fn send_signal() -> Weight;
    fn send_service_signal() -> Weight;
}

/// Weights for pallet_signal using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: Signal RatingSignalList (r:0 w:1)
    /// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
    fn send_rating_signal() -> Weight {
        Weight::from_parts(10_000_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: Signal RatingSignalList (r:0 w:1)
    /// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
    fn update_rating_signal() -> Weight {
        Weight::from_parts(10_000_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: Signal RatingSignalList (r:0 w:1)
    /// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
    fn revoke_rating_signal() -> Weight {
        Weight::from_parts(10_000_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn send_signal() -> Weight {
        Weight::from_parts(10_000_000, 0)
    }
    fn send_service_signal() -> Weight {
        Weight::from_parts(16_000_000, 0)
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: Signal RatingSignalList (r:0 w:1)
    /// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
    fn send_rating_signal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 9_000_000 picoseconds.
        Weight::from_parts(10_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: Signal RatingSignalList (r:0 w:1)
    /// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
    fn update_rating_signal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 9_000_000 picoseconds.
        Weight::from_parts(10_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: Signal RatingSignalList (r:0 w:1)
    /// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
    fn revoke_rating_signal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 9_000_000 picoseconds.
        Weight::from_parts(10_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn send_signal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 7_000_000 picoseconds.
        Weight::from_parts(10_000_000, 0)
    }
    fn send_service_signal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 8_000_000 picoseconds.
        Weight::from_parts(16_000_000, 0)
    }
}
