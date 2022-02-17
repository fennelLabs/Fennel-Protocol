
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight, Weight};

pub trait WeightInfo {
    fn issue_key() -> Weight;
    fn announce_key() -> Weight;
    fn revoke_key() -> Weight;
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
