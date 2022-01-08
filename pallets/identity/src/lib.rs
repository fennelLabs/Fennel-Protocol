#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use frame_support::inherent::Vec;
    use codec::alloc::collections::BTreeSet;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 { 0 }

    #[pallet::storage]
    #[pallet::getter(fn identity_number)]
    /// Tracks the number of identities currently active on the network.
    pub type IdentityNumber<T: Config> =  StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn revoked_identity_number)]
    /// Tracks the number of identities currently active on the network.
    pub type RevokedIdentityNumber<T: Config> =  StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn identity_list)]
    /// Maps accounts to the array of identities it owns.
    pub type IdentityList<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BTreeSet<u32>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn identity_trait_list)]
    /// Maps identity ID numbers to their key/value attributes.
    pub type IdentityTraitList<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, Vec<u8>, Vec<u8>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Announce a new identity to the network. Contains the ID number of the identity and
        /// the owning AccountId. 
        IdentityCreated(u32, T::AccountId),
        /// Announce that an identity has been revoked. Contains the ID number of the identity ad
        /// the owning AccountId.
        IdentityRevoked(u32, T::AccountId),
        /// Announce that an identity has been updated. Contains the ID number of the identity and
        /// the owning AccountId.
        IdentityUpdated(u32, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        /// Create a new identity owned by origin.
        pub fn create_identity(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let new_id: u32 = <IdentityNumber<T>>::get().checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

            <IdentityList<T>>::try_mutate(&who, |ids| -> DispatchResult {
                ids.insert(new_id);
                Ok(())
            })?;

            <IdentityNumber<T>>::put(new_id);
            Self::deposit_event(Event::IdentityCreated(new_id, who));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        /// Revokes the identity with ID number identity_id, as long as the identity is owned by
        /// origin.
        pub fn revoke_identity(origin: OriginFor<T>, identity_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let new_total = <RevokedIdentityNumber<T>>::get().checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

            let successfully_removed = <IdentityList<T>>::try_mutate(&who, |ids| -> Result<bool, DispatchError> {
                Ok(ids.remove(&identity_id))
            })?;

            if successfully_removed {
                <RevokedIdentityNumber<T>>::put(new_total);
                Self::deposit_event(Event::IdentityRevoked(identity_id, who));
            }

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        /// Add a new identity trait to identity_id with key/value.
        pub fn add_identity_trait(origin: OriginFor<T>, identity_id: u32, key: Vec<u8>, value: Vec<u8>) -> DispatchResult {
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        /// Remove an identity trait named by trait_name from the identity with ID identity_id.
        pub fn remove_identity_trait(origin: OriginFor<T>) -> DispatchResult {
            Ok(())
        }
    }
}
