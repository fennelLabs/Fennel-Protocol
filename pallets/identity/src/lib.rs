#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use pallet::*;
use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::One;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        // Please add one line of comment here about this config
        type MaxSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 {
        0
    }

    #[pallet::storage]
    #[pallet::getter(fn identity_number)]
    /// Tracks the number of identities currently active on the network.
    pub type IdentityNumber<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_signal_count)]
    /// Tracks the number of signals transmitted to the network.
    pub type SignalCount<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn identity_list)]
    /// Maps accounts to the array of identities it owns.
    pub type IdentityList<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn identity_trait_list)]
    /// Maps identity ID numbers to their key/value attributes.
    pub type IdentityTraitList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        BoundedVec<u8, T::MaxSize>,
        ValueQuery,
    >;

    #[pallet::event]
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
    #[derive(PartialEq, Eq)]
    pub enum Error<T> {
        /// The provided value is too large.
        StorageOverflow,
        /// The current account does not own the identity.
        IdentityNotOwned,
    }

    impl<T: Config> Pallet<T> {
        fn is_identity_owned_by_sender(account_id: &T::AccountId, identity_id: &u32) -> bool {
            match <IdentityList<T>>::try_get(identity_id) {
                Result::Ok(owner) => owner == *account_id,
                Result::Err(_) => false,
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new identity owned by origin.
        #[pallet::weight(T::WeightInfo::create_identity())]
        #[pallet::call_index(0)]
        pub fn create_identity(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let current_id: u32 = <IdentityNumber<T>>::get();
            <IdentityNumber<T>>::try_mutate(|current_id| -> DispatchResult {
                *current_id =
                    current_id.checked_add(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                Ok(())
            })?;
            let new_id: u32 = <IdentityNumber<T>>::get();

            // Ensure that the index current_id isn't already in use.
            ensure!(!<IdentityList<T>>::contains_key(&current_id), Error::<T>::StorageOverflow);

            <IdentityList<T>>::try_mutate(&current_id, |owner| -> DispatchResult {
                *owner = Some(who.clone());
                Ok(())
            })?;

            <IdentityNumber<T>>::put(new_id);
            Self::deposit_event(Event::IdentityCreated(current_id, who));

            Ok(())
        }

        /// Revokes the identity with ID number identity_id, as long as the identity is owned by
        /// origin.
        #[pallet::weight(T::WeightInfo::revoke_identity())]
        #[pallet::call_index(1)]
        pub fn revoke_identity(origin: OriginFor<T>, identity_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Make sure the identity is owned by the sender.
            ensure!(
                Self::is_identity_owned_by_sender(&who, &identity_id),
                Error::<T>::IdentityNotOwned
            );

            <IdentityList<T>>::try_mutate(&identity_id, |owner| -> DispatchResult {
                *owner = None;
                Ok(())
            })?;

            Self::deposit_event(Event::IdentityRevoked(identity_id, who));

            Ok(())
        }

        /// Add a new identity trait to identity_id with key/value.
        #[pallet::weight(T::WeightInfo::add_or_update_identity_trait())]
        #[pallet::call_index(2)]
        pub fn add_or_update_identity_trait(
            origin: OriginFor<T>,
            identity_id: u32,
            key: BoundedVec<u8, T::MaxSize>,
            value: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Self::is_identity_owned_by_sender(&who, &identity_id),
                Error::<T>::IdentityNotOwned
            );

            <IdentityTraitList<T>>::try_mutate(identity_id, key, |v| -> DispatchResult {
                *v = value;
                Ok(())
            })?;

            Self::deposit_event(Event::IdentityUpdated(identity_id, who));

            Ok(())
        }

        /// Remove an identity trait named by trait_name from the identity with ID identity_id.
        #[pallet::weight(T::WeightInfo::remove_identity_trait())]
        #[pallet::call_index(3)]
        pub fn remove_identity_trait(
            origin: OriginFor<T>,
            identity_id: u32,
            key: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Self::is_identity_owned_by_sender(&who, &identity_id),
                Error::<T>::IdentityNotOwned
            );

            <IdentityTraitList<T>>::remove(identity_id, key);
            Self::deposit_event(Event::IdentityUpdated(identity_id, who));

            Ok(())
        }
    }
}
