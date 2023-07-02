//! Autogenerated weights for `pallet_trust`
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
// --pallet=pallet_trust
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

/// Weight functions needed for pallet_trust.
pub trait WeightInfo {
    fn issue_trust() -> Weight;
    fn revoke_trust() -> Weight;
    fn remove_trust() -> Weight;
    fn request_trust() -> Weight;
    fn remove_revoked_trust() -> Weight;
    fn cancel_trust_request() -> Weight;
}

/// Weights for pallet_trust using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: Trust TrustIssuance (r:1 w:1)
    /// Proof: Trust TrustIssuance (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentIssued (r:1 w:1)
    /// Proof: Trust CurrentIssued (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn issue_trust() -> Weight {
        Weight::from_parts(14_000_000, 3565)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRevocation (r:1 w:1)
    /// Proof: Trust TrustRevocation (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRevoked (r:1 w:1)
    /// Proof: Trust CurrentRevoked (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn revoke_trust() -> Weight {
        Weight::from_parts(14_000_000, 3565)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustIssuance (r:1 w:1)
    /// Proof: Trust TrustIssuance (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentIssued (r:1 w:1)
    /// Proof: Trust CurrentIssued (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn remove_trust() -> Weight {
        Weight::from_parts(16_000_000, 3565)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRequestList (r:1 w:1)
    /// Proof: Trust TrustRequestList (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRequests (r:1 w:1)
    /// Proof: Trust CurrentRequests (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn request_trust() -> Weight {
        Weight::from_parts(14_000_000, 3565)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRevocation (r:1 w:1)
    /// Proof: Trust TrustRevocation (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRevoked (r:1 w:1)
    /// Proof: Trust CurrentRevoked (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn remove_revoked_trust() -> Weight {
        Weight::from_parts(17_000_000, 3565)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRequestList (r:1 w:1)
    /// Proof: Trust TrustRequestList (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRequests (r:1 w:1)
    /// Proof: Trust CurrentRequests (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn cancel_trust_request() -> Weight {
        Weight::from_parts(16_000_000, 3565)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: Trust TrustIssuance (r:1 w:1)
    /// Proof: Trust TrustIssuance (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentIssued (r:1 w:1)
    /// Proof: Trust CurrentIssued (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn issue_trust() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `3565`
        // Minimum execution time: 13_000_000 picoseconds.
        Weight::from_parts(14_000_000, 3565)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRevocation (r:1 w:1)
    /// Proof: Trust TrustRevocation (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRevoked (r:1 w:1)
    /// Proof: Trust CurrentRevoked (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn revoke_trust() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `3565`
        // Minimum execution time: 13_000_000 picoseconds.
        Weight::from_parts(14_000_000, 3565)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustIssuance (r:1 w:1)
    /// Proof: Trust TrustIssuance (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentIssued (r:1 w:1)
    /// Proof: Trust CurrentIssued (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn remove_trust() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `157`
        //  Estimated: `3565`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(16_000_000, 3565)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRequestList (r:1 w:1)
    /// Proof: Trust TrustRequestList (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRequests (r:1 w:1)
    /// Proof: Trust CurrentRequests (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn request_trust() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `3565`
        // Minimum execution time: 13_000_000 picoseconds.
        Weight::from_parts(14_000_000, 3565)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRevocation (r:1 w:1)
    /// Proof: Trust TrustRevocation (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRevoked (r:1 w:1)
    /// Proof: Trust CurrentRevoked (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn remove_revoked_trust() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `157`
        //  Estimated: `3565`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3565)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: Trust TrustRequestList (r:1 w:1)
    /// Proof: Trust TrustRequestList (max_values: None, max_size: Some(100), added: 2575, mode:
    /// MaxEncodedLen) Storage: Trust CurrentRequests (r:1 w:1)
    /// Proof: Trust CurrentRequests (max_values: Some(1), max_size: Some(4), added: 499, mode:
    /// MaxEncodedLen)
    fn cancel_trust_request() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `155`
        //  Estimated: `3565`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(16_000_000, 3565)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
}
