#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions for pallet_signal.
pub trait WeightInfo {
    fn issue_key() -> Weight;
    fn announce_key() -> Weight;
    fn revoke_key() -> Weight;
}

pub struct SubstrateWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeights<T> {
	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn issue_key() -> Weight {
		(10_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    fn announce_key() -> Weight {
		(10_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    fn revoke_key() -> Weight {
		(10_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

impl WeightInfo for () {
    fn issue_key() -> Weight {
        (10_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn announce_key() -> Weight {
        (10_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn revoke_key() -> Weight {
        (10_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
}
