
//! Autogenerated weights for `pallet_fennel_identity`
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
// --pallet=pallet_fennel_identity
// --extrinsic=*
// --steps=100
// --repeat=10
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_fennel_identity` using the recommended hardware.
pub struct WeightInfo<T>(pub PhantomData<T>);
impl<T: frame_system::Config> pallet_fennel_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity IdentityNumber (r:1 w:1)
	// Storage: Identity IdentityList (r:1 w:1)
	fn create_identity(_s: u32, ) -> Weight {
		Weight::from_ref_time(17_883_729 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity RevokedIdentityNumber (r:1 w:1)
	// Storage: Identity IdentityList (r:1 w:1)
	fn revoke_identity(_s: u32, ) -> Weight {
		Weight::from_ref_time(19_541_683 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityList (r:1 w:0)
	// Storage: Identity IdentityTraitList (r:1 w:1)
	fn add_or_update_identity_trait(_s: u32, ) -> Weight {
		Weight::from_ref_time(19_080_858 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityList (r:1 w:0)
	// Storage: Identity IdentityTraitList (r:0 w:1)
	fn remove_identity_trait(s: u32, ) -> Weight {
		Weight::from_ref_time(18_171_518 as u64)
			// Standard Error: 814
			.saturating_add(Weight::from_ref_time(4_144 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityList (r:1 w:0)
	// Storage: Identity SignalCount (r:1 w:1)
	// Storage: Identity SignatureSignal (r:0 w:1)
	fn sign_for_identity(_s: u32, ) -> Weight {
		Weight::from_ref_time(21_634_884 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
