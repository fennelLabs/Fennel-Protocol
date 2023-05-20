
//! Autogenerated weights for `pallet_keystore`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_keystore
// --extrinsic=*
// --steps=100
// --repeat=10
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_keystore` using the recommended hardware.
pub struct WeightInfo<T>(pub PhantomData<T>);
impl<T: frame_system::Config> pallet_keystore::WeightInfo for WeightInfo<T> {
	// Storage: Keystore IssuedKeys (r:0 w:1)
	fn announce_key(_s: u32, ) -> Weight {
		Weight::from_ref_time(14_470_561 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Keystore IssuedKeys (r:0 w:1)
	fn revoke_key(s: u32, ) -> Weight {
		Weight::from_ref_time(13_993_102 as u64)
			// Standard Error: 408
			.saturating_add(Weight::from_ref_time(72 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Keystore IssuedEncryptionKeys (r:0 w:1)
	fn issue_encryption_key(_s: u32, ) -> Weight {
		Weight::from_ref_time(13_247_326 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}