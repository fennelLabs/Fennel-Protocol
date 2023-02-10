#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight, Weight};
use core::marker::PhantomData;

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
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule RevokedIdentityNumber (r:1 w:1)
    // Storage: IdentityModule IdentityList (r:1 w:1)
    fn revoke_identity() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:1 w:1)
    fn add_or_update_identity_trait() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:0 w:1)
    fn remove_identity_trait() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule SignalCount (r:1 w:1)
    // Storage: IdentityModule FennelSignal (r:0 w:1)
    fn sign_for_identity() -> Weight {
        Weight::from_ref_time(0)
    }
}

pub struct SubstrateWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeights<T> {
    // Storage: IdentityModule IdentityNumber (r:1 w:1)
    // Storage: IdentityModule IdentityList (r:1 w:1)
    fn create_identity() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule RevokedIdentityNumber (r:1 w:1)
    // Storage: IdentityModule IdentityList (r:1 w:1)
    fn revoke_identity() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:1 w:1)
    fn add_or_update_identity_trait() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule IdentityTraitList (r:0 w:1)
    fn remove_identity_trait() -> Weight {
        Weight::from_ref_time(0)
    }
    // Storage: IdentityModule IdentityList (r:1 w:0)
    // Storage: IdentityModule SignalCount (r:1 w:1)
    // Storage: IdentityModule FennelSignal (r:0 w:1)
    fn sign_for_identity() -> Weight {
        Weight::from_ref_time(0)
    }
}
