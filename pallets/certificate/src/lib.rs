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
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use crate::{weights::WeightInfo, CERTIFICATE_EXISTS};

    const LOCK_ID: LockIdentifier = *b"certlock";

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 {
        0
    }

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
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CertificateSent(T::AccountId, T::AccountId),
        CertificateRevoked(T::AccountId, T::AccountId),
        CertificateLock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
        CertificateUnlock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        CertificateNotOwned,
        CertificateExists,
        InsufficientBalance,
    }

    impl<T: Config> Pallet<T> {
        fn is_certificate_owned_by_sender(
            account_id: &T::AccountId,
            recipient_id: &T::AccountId,
        ) -> bool {
            <CertificateList<T>>::try_get(account_id, recipient_id).is_ok()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a Certificate payload defined as part of the transaction
        /// and commits the details to storage.
        #[pallet::weight(<T as Config>::WeightInfo::send_certificate())]
        #[pallet::call_index(0)]
        pub fn send_certificate(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into())
            }

            ensure!(
                !Self::is_certificate_owned_by_sender(&who, &recipient),
                Error::<T>::CertificateExists
            );
            // Insert a placeholder value into storage - if the pair (who, recipient) exists, we
            // know there's a certificate present for the pair, regardless of value.
            T::Currency::set_lock(LOCK_ID, &who, 10u32.into(), WithdrawReasons::all());

            Self::deposit_event(Event::CertificateLock(
                who.clone(),
                T::Currency::free_balance(&who),
            ));

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

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into())
            }

            ensure!(
                Self::is_certificate_owned_by_sender(&who, &recipient),
                Error::<T>::CertificateNotOwned
            );

            T::Currency::remove_lock(LOCK_ID, &who);
            Self::deposit_event(Event::CertificateUnlock(
                who.clone(),
                T::Currency::free_balance(&who),
            ));

            <CertificateList<T>>::remove(&who, recipient.clone());

            Self::deposit_event(Event::CertificateRevoked(recipient, who));

            Ok(())
        }
    }
}
