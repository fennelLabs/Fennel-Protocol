
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-27, STEPS: `100`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_identity
// --extrinsic=*
// --steps=100
// --repeat=100
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn create_identity() -> Weight;
	fn revoke_identity() -> Weight;
	fn revoke_identity_heavy_storage() -> Weight;
	fn add_or_update_identity_trait() -> Weight;
	fn add_or_update_long_identity_trait() -> Weight;
	fn add_or_update_many_identity_traits() -> Weight;
	fn remove_identity_trait() -> Weight;
	fn remove_identity_trait_heavy_storage() -> Weight;
	fn remove_long_identity_trait() -> Weight;
	fn sign_for_identity() -> Weight;
	fn sign_for_identity_big_vector() -> Weight;
	fn sign_for_identity_many_times() -> Weight;
}

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fennel_identity::WeightInfo for SubstrateWeight<T> {
	/// Storage: Identity IdentityNumber (r:1 w:1)
	/// Proof: Identity IdentityNumber (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityList (r:1 w:1)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	fn create_identity() -> Weight {
		Weight::from_parts(67_000_000, 7574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity RevokedIdentityNumber (r:1 w:1)
	/// Proof: Identity RevokedIdentityNumber (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityList (r:1 w:1)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	fn revoke_identity() -> Weight {
		Weight::from_parts(15_000_000, 3574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity RevokedIdentityNumber (r:1 w:1)
	/// Proof: Identity RevokedIdentityNumber (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityList (r:1 w:1)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	fn revoke_identity_heavy_storage() -> Weight {
		Weight::from_parts(61_000_000, 7574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:1 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn add_or_update_identity_trait() -> Weight {
		Weight::from_parts(15_000_000, 3574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:1 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn add_or_update_long_identity_trait() -> Weight {
		Weight::from_parts(18_000_000, 3574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:1 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn add_or_update_many_identity_traits() -> Weight {
		Weight::from_parts(52_000_000, 4182)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:0 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn remove_identity_trait() -> Weight {
		Weight::from_parts(14_000_000, 3608)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:0 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn remove_identity_trait_heavy_storage() -> Weight {
		Weight::from_parts(45_000_000, 3608)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:0 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn remove_long_identity_trait() -> Weight {
		Weight::from_parts(16_000_000, 3608)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity SignalCount (r:1 w:1)
	/// Proof: Identity SignalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity SignatureSignal (r:0 w:1)
	/// Proof Skipped: Identity SignatureSignal (max_values: None, max_size: None, mode: Measured)
	fn sign_for_identity() -> Weight {
		Weight::from_parts(18_000_000, 3574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity SignalCount (r:1 w:1)
	/// Proof: Identity SignalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity SignatureSignal (r:0 w:1)
	/// Proof Skipped: Identity SignatureSignal (max_values: None, max_size: None, mode: Measured)
	fn sign_for_identity_big_vector() -> Weight {
		Weight::from_parts(19_000_000, 3574)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity SignalCount (r:1 w:1)
	/// Proof: Identity SignalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity SignatureSignal (r:0 w:1)
	/// Proof Skipped: Identity SignatureSignal (max_values: None, max_size: None, mode: Measured)
	fn sign_for_identity_many_times() -> Weight {
		Weight::from_parts(48_000_000, 3631)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Identity IdentityNumber (r:1 w:1)
	/// Proof: Identity IdentityNumber (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityList (r:1 w:1)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	fn create_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4109`
		//  Estimated: `7574`
		// Minimum execution time: 56_000_000 picoseconds.
		Weight::from_parts(67_000_000, 7574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity RevokedIdentityNumber (r:1 w:1)
	/// Proof: Identity RevokedIdentityNumber (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityList (r:1 w:1)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	fn revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(15_000_000, 3574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity RevokedIdentityNumber (r:1 w:1)
	/// Proof: Identity RevokedIdentityNumber (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityList (r:1 w:1)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	fn revoke_identity_heavy_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4109`
		//  Estimated: `7574`
		// Minimum execution time: 50_000_000 picoseconds.
		Weight::from_parts(61_000_000, 7574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:1 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn add_or_update_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 3574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:1 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn add_or_update_long_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 3574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:1 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn add_or_update_many_identity_traits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `717`
		//  Estimated: `4182`
		// Minimum execution time: 47_000_000 picoseconds.
		Weight::from_parts(52_000_000, 4182)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:0 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn remove_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `143`
		//  Estimated: `3608`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3608)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:0 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn remove_identity_trait_heavy_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `143`
		//  Estimated: `3608`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(45_000_000, 3608)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityTraitList (r:0 w:1)
	/// Proof Skipped: Identity IdentityTraitList (max_values: None, max_size: None, mode: Measured)
	fn remove_long_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `143`
		//  Estimated: `3608`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 3608)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity SignalCount (r:1 w:1)
	/// Proof: Identity SignalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity SignatureSignal (r:0 w:1)
	/// Proof Skipped: Identity SignatureSignal (max_values: None, max_size: None, mode: Measured)
	fn sign_for_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 3574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity SignalCount (r:1 w:1)
	/// Proof: Identity SignalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity SignatureSignal (r:0 w:1)
	/// Proof Skipped: Identity SignatureSignal (max_values: None, max_size: None, mode: Measured)
	fn sign_for_identity_big_vector() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 3574)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityList (r:1 w:0)
	/// Proof Skipped: Identity IdentityList (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity SignalCount (r:1 w:1)
	/// Proof: Identity SignalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity SignatureSignal (r:0 w:1)
	/// Proof Skipped: Identity SignatureSignal (max_values: None, max_size: None, mode: Measured)
	fn sign_for_identity_many_times() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `3631`
		// Minimum execution time: 43_000_000 picoseconds.
		Weight::from_parts(48_000_000, 3631)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
