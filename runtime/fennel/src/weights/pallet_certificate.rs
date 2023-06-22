
//! Autogenerated weights for `pallet_certificate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_certificate
// --extrinsic=*
// --steps=100
// --repeat=10
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_certificate.
pub trait WeightInfo {
	fn send_certificate() -> Weight;
	fn revoke_certificate() -> Weight;
}

/// Weights for pallet_certificate using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Certificate CertificateList (r:0 w:1)
	/// Proof Skipped: Certificate CertificateList (max_values: None, max_size: None, mode: Measured)
	fn send_certificate() -> Weight {
		Weight::from_parts(10_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Certificate CertificateList (r:1 w:1)
	/// Proof Skipped: Certificate CertificateList (max_values: None, max_size: None, mode: Measured)
	fn revoke_certificate() -> Weight {
		Weight::from_parts(17_000_000, 3668)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Certificate CertificateList (r:0 w:1)
	/// Proof Skipped: Certificate CertificateList (max_values: None, max_size: None, mode: Measured)
	fn send_certificate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Certificate CertificateList (r:1 w:1)
	/// Proof Skipped: Certificate CertificateList (max_values: None, max_size: None, mode: Measured)
	fn revoke_certificate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `3668`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(17_000_000, 3668)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}