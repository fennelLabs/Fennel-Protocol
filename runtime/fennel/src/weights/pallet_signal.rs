
//! Autogenerated weights for `pallet_signal`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-10, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --repeat=10
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_signal` using the recommended hardware.
pub struct WeightInfo<T>(pub PhantomData<T>);
impl<T: frame_system::Config> pallet_signal::WeightInfo for WeightInfo<T> {
	// Storage: Signal SignalParameterList (r:0 w:1)
	fn set_signal_parameter() -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Signal RatingSignalList (r:0 w:1)
	fn send_rating_signal() -> Weight {
		Weight::from_ref_time(34_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Signal WhiteflagRatingSignalList (r:0 w:1)
	fn send_whiteflag_rating_signal() -> Weight {
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Signal RatingSignalList (r:0 w:1)
	fn update_rating_signal() -> Weight {
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Signal WhiteflagRatingSignalList (r:0 w:1)
	fn update_whiteflag_rating_signal() -> Weight {
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Signal RatingSignalList (r:0 w:1)
	fn revoke_rating_signal() -> Weight {
		Weight::from_ref_time(42_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn send_signal() -> Weight {
		Weight::from_ref_time(13_000_000 as u64)
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Signal WhiteflagRatingSignalList (r:0 w:1)
	fn revoke_whiteflag_rating_signal() -> Weight {
		Weight::from_ref_time(41_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn send_service_signal() -> Weight {
		Weight::from_ref_time(13_000_000 as u64)
	}
}
