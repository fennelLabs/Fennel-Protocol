#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    use crate::weights::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 {
        0
    }

    #[pallet::storage]
    #[pallet::getter(fn certificate_number)]
    /// Tracks the number of identities currently active on the network.
    pub type CertificateNumber<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn certificate_list)]
    /// Maps accounts to the array of identities it owns.
    pub type CertificateList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        u32,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CertificateSent(T::AccountId, T::AccountId),
        CertificateRevoked(T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        CertificateNotOwned,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a Certificate payload defined as part of the transaction
        /// and commits the details to storage.
        #[pallet::weight(<T as Config>::WeightInfo::send_certificate())]
        pub fn send_certificate(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <CertificateList<T>>::insert(&who, recipient.clone(), 0);

            Self::deposit_event(Event::CertificateSent(recipient, who));

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::revoke_certificate())]
        /// Revokes the identity with ID number identity_id, as long as the identity is owned by
        /// origin.
        pub fn revoke_certificate(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <CertificateList<T>>::remove(&who, recipient.clone());

            Self::deposit_event(Event::CertificateRevoked(recipient, who));

            Ok(())
        }
    }
}
