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
    use codec::alloc::collections::BTreeSet;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
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
    #[pallet::getter(fn revoked_identity_number)]
    /// Tracks the number of identities currently active on the network.
    pub type RevokedIdentityNumber<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_signal_count)]
    /// Tracks the number of signals transmitted to the network.
    pub type SignalCount<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn identity_list)]
    /// Maps accounts to the array of identities it owns.
    pub type IdentityList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BTreeSet<u32>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
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

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn get_signal_record)]
    /// Tracks all signals sent by an identity.
    pub type SignatureSignal<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32,
        Blake2_128Concat,
        u32,
        BoundedVec<u8, T::MaxSize>,
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
        /// Announce that the given identity, owned by the given AccountId, has signed a signal
        /// containing the given vector.
        SignedSignal(u32, T::AccountId, BoundedVec<u8, T::MaxSize>),
    }

    #[pallet::error]
    #[derive(PartialEq, Eq)]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        IdentityNotOwned,
    }

    impl<T: Config> Pallet<T> {
        fn is_identity_owned_by_sender(account_id: &T::AccountId, identity_id: &u32) -> bool {
            match <IdentityList<T>>::try_get(account_id) {
                Result::Ok(ids) => ids.contains(identity_id),
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
                *current_id = current_id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                Ok(())
            })?;
            let new_id: u32 = <IdentityNumber<T>>::get();

            <IdentityList<T>>::try_mutate(&who, |ids| -> DispatchResult {
                ids.insert(current_id);
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
            <RevokedIdentityNumber<T>>::try_mutate(|current_id| -> DispatchResult {
                *current_id = current_id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                Ok(())
            })?;

            let new_total = <RevokedIdentityNumber<T>>::get();

            <IdentityList<T>>::try_mutate(&who, |ids| -> DispatchResult {
                ensure!(ids.remove(&identity_id), Error::<T>::IdentityNotOwned);
                Ok(())
            })?;

            <RevokedIdentityNumber<T>>::put(new_total);
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
        /// Issue a signed Fennel signal on behalf of an owned identity.
        #[pallet::weight(T::WeightInfo::sign_for_identity())]
        #[pallet::call_index(4)]
        pub fn sign_for_identity(
            origin: OriginFor<T>,
            identity_id: u32,
            content: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Self::is_identity_owned_by_sender(&who, &identity_id),
                Error::<T>::IdentityNotOwned
            );
            let signal_id: u32 = <SignalCount<T>>::get();
            <SignalCount<T>>::try_mutate(|signal_id| -> DispatchResult {
                *signal_id = signal_id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                Ok(())
            })?;
            let new_id: u32 = <SignalCount<T>>::get();
            <SignatureSignal<T>>::insert(&identity_id, &signal_id, &content);
            <SignalCount<T>>::put(new_id);
            Self::deposit_event(Event::SignedSignal(identity_id, who, content));

            Ok(())
        }
    }
}
