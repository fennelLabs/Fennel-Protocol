
//! Autogenerated weights for `pallet_trust`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-16, STEPS: `20`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-fennel-protocol
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_trust
// --extrinsic
// *
// --steps
// 20
// --repeat
// 100
// --raw
// --output
// ./


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn send_certificate() -> Weight;
    fn revoke_certificate() -> Weight;
}

/// Weight functions for pallet_trust.
pub struct SubstrateWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeights<T> {
	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn send_certificate() -> Weight {
        Weight::from_ref_time(0)
	}

    fn revoke_certificate() -> Weight {
        Weight::from_ref_time(0)
    }
}

impl WeightInfo for () {
	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn send_certificate() -> Weight {
        Weight::from_ref_time(0)
	}

    fn revoke_certificate() -> Weight {
        Weight::from_ref_time(0)
    }
}
