#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

/// Weight functions for pallet_fennel_identity.
pub trait WeightInfo {
    fn create_identity() -> Weight;
    fn revoke_identity() -> Weight;
    fn add_or_update_identity_trait() -> Weight;
    fn remove_identity_trait() -> Weight;
    fn sign_for_identity() -> Weight;
}

impl WeightInfo for () {
    // Storage: IdentityModule IdentityNumber (r:1 w:1)
    // Storage: IdentityModule IdentityList (r:1 w:1)
    fn create_identity() -> Weight {
        (47_568_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: IdentityModule RevokedIdentityNumber (r:1 w:1)
    // Storage: IdentityModule IdentityList (r:1 w:1)
    fn revoke_identity() -> Weight {
        (43_296_000 as Weight)
            // Standard Error: 6_000
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:1 w:1)
    fn add_or_update_identity_trait() -> Weight {
        (48_308_000 as Weight)
            // Standard Error: 9_000
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:0 w:1)
    fn remove_identity_trait() -> Weight {
        (48_195_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule SignalCount (r:1 w:1)
    // Storage: IdentityModule FennelSignal (r:0 w:1)
    fn sign_for_identity() -> Weight {
        (51_112_000 as Weight)
            // Standard Error: 10_000
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}
