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

const CERTIFICATE_EXISTS: bool = true;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    use crate::{weights::WeightInfo, CERTIFICATE_EXISTS};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn certificate_list)]
    /// Maps accounts to the array of identities it owns.
    pub type CertificateList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A `certificate` was sent.
        CertificateSent(T::AccountId, T::AccountId),
        /// A `certificate` was revoked.
        CertificateRevoked(T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The current account does not own the certificate.
        CertificateNotOwned,
        /// The certificate already exists.
        CertificateExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a Certificate payload defined as part of the transaction
        /// and commits the details to storage.
        #[pallet::weight(T::WeightInfo::send_certificate())]
        #[pallet::call_index(0)]
        pub fn send_certificate(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                !CertificateList::<T>::contains_key(&who, &recipient),
                Error::<T>::CertificateExists
            );
            // Insert a placeholder value into storage - if the pair (who, recipient) exists, we
            // know there's a certificate present for the pair, regardless of value.
            <CertificateList<T>>::insert(&who, recipient.clone(), CERTIFICATE_EXISTS);

            Self::deposit_event(Event::CertificateSent(recipient, who));

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::revoke_certificate())]
        #[pallet::call_index(1)]
        /// Revokes the identity with ID number identity_id, as long as the identity is owned by
        /// origin.
        pub fn revoke_certificate(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                CertificateList::<T>::contains_key(&who, &recipient),
                Error::<T>::CertificateNotOwned
            );

            <CertificateList<T>>::remove(&who, recipient.clone());

            Self::deposit_event(Event::CertificateRevoked(recipient, who));

            Ok(())
        }
    }
}
