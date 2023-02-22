
//! Autogenerated weights for pallet_trust
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `kusanagi`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// /Users/andrewplaza/Projects/fennel/Fennel-Protocol/target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --steps=50
// --repeat=20
// --pallet=pallet_trust
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=src/trust_weights.rs
// --template=/Users/andrewplaza/Projects/fennel/Fennel-Protocol/scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_trust.
pub trait WeightInfo {
	fn issue_trust() -> Weight;
	fn revoke_trust() -> Weight;
	fn remove_trust() -> Weight;
	fn remove_revoked_trust() -> Weight;
	fn request_trust() -> Weight;
	fn cancel_trust_request() -> Weight;
}

/// Weights for pallet_trust using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Trust TrustIssuance (r:1 w:1)
	// Storage: Trust CurrentIssued (r:1 w:1)
	fn issue_trust() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(18_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Trust TrustRevocation (r:1 w:1)
	// Storage: Trust CurrentRevoked (r:1 w:1)
	fn revoke_trust() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(17_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Trust TrustIssuance (r:1 w:0)
	fn remove_trust() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Trust TrustRevocation (r:1 w:0)
	fn remove_revoked_trust() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Trust TrustRequestList (r:1 w:1)
	// Storage: Trust CurrentRequests (r:1 w:1)
	fn request_trust() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(18_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Trust TrustRequestList (r:1 w:0)
	fn cancel_trust_request() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Trust TrustIssuance (r:1 w:1)
	// Storage: Trust CurrentIssued (r:1 w:1)
	fn issue_trust() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(18_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Trust TrustRevocation (r:1 w:1)
	// Storage: Trust CurrentRevoked (r:1 w:1)
	fn revoke_trust() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(17_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Trust TrustIssuance (r:1 w:0)
	fn remove_trust() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
	}
	// Storage: Trust TrustRevocation (r:1 w:0)
	fn remove_revoked_trust() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
	}
	// Storage: Trust TrustRequestList (r:1 w:1)
	// Storage: Trust CurrentRequests (r:1 w:1)
	fn request_trust() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(18_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Trust TrustRequestList (r:1 w:0)
	fn cancel_trust_request() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
	}
}
