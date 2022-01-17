#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

/// Weight functions for pallet_fennel_identity.
pub trait WeightInfo {
    fn create_identity() -> Weight;
    fn add_or_update_identity_trait() -> Weight;
    fn remove_identity_trait() -> Weight;
}

impl WeightInfo for () {
    // Storage: IdentityModule IdentityNumber (r:1 w:1)
    // Storage: IdentityModule IdentityList (r:1 w:1)
    fn create_identity() -> Weight {
        (45_239_000 as Weight)
            // Standard Error: 26_000
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }

    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:1 w:1)
    fn add_or_update_identity_trait() -> Weight {
        (49_785_000 as Weight)
            // Standard Error: 38_000
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }

    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:0 w:1)
    fn remove_identity_trait() -> Weight {
        (50_813_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}