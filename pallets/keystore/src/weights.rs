
//! Autogenerated weights for pallet_keystore
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
// --pallet=pallet_keystore
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=src/keystore_weights.rs
// --template=/Users/andrewplaza/Projects/fennel/Fennel-Protocol/scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_keystore.
pub trait WeightInfo {
	fn announce_key() -> Weight;
	fn revoke_key() -> Weight;
	fn issue_encryption_key() -> Weight;
}

/// Weights for pallet_keystore using the Substrate node and recommended hardware.
pub struct SubstrateWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeights<T> {
	// Storage: Keystore IssuedKeys (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn announce_key() -> Weight {
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(14_044_685)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Keystore IssuedKeys (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn revoke_key() -> Weight {
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(13_778_741)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Keystore IssuedEncryptionKeys (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn issue_encryption_key() -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(13_025_278)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Keystore IssuedKeys (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn announce_key() -> Weight {
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(14_044_685)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Keystore IssuedKeys (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn revoke_key() -> Weight {
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(13_778_741)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Keystore IssuedEncryptionKeys (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn issue_encryption_key() -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(13_025_278)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
