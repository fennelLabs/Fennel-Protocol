#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight, Weight};

/// Weight functions for pallet_fennel_identity.
pub trait WeightInfo {
    fn revoke_key() -> Weight;
}

impl WeightInfo for () {
    fn revoke_key() -> Weight {
        (47_568_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    } 
}
