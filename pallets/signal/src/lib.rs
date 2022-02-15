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
    use super::*;
    use frame_support::{dispatch::DispatchResult, inherent::Vec, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_issued_key)]
    pub type IssuedKeys<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, Vec<u8>, Vec<u8>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KeyIssued(Vec<u8>, T::AccountId),
        KeyRevoked(Vec<u8>, T::AccountId),
        KeyExists(Vec<u8>, Vec<u8>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Allows an account `origin` to announce a key with `fingerprint` hosted at `location`
        #[pallet::weight(<T as Config>::WeightInfo::issue_key())]
        pub fn issue_key(
            origin: OriginFor<T>,
            fingerprint: Vec<u8>,
            location: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <IssuedKeys<T>>::insert(&who, &fingerprint, location);

            Self::deposit_event(Event::KeyIssued(fingerprint, who));
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::announce_key())]
        pub fn announce_key(
            origin: OriginFor<T>,
            fingerprint: Vec<u8>,
            location: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <IssuedKeys<T>>::insert(&who, &fingerprint, &location);

            Self::deposit_event(Event::KeyExists(fingerprint, location, who));
            Ok(())
        }

        /// Announces that `origin`'s key `fingerprint` has been revoked
        #[pallet::weight(<T as Config>::WeightInfo::revoke_key())]
        pub fn revoke_key(origin: OriginFor<T>, fingerprint: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <IssuedKeys<T>>::remove(&who, &fingerprint);

            Self::deposit_event(Event::KeyRevoked(fingerprint, who));
            Ok(())
        }
    }
}
