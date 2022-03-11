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
    use frame_support::{dispatch::DispatchResult, inherent::Vec, pallet_prelude::*};
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

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SignalSent(Vec<u8>, T::AccountId),
        ServiceSignalSent(Vec<u8>, Vec<u8>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a signal payload defined as part of the transaction
        /// without relying on storage.
        #[pallet::weight(<T as Config>::WeightInfo::send_signal())]
        pub fn send_signal(origin: OriginFor<T>, signal: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::SignalSent(signal, who));
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::send_service_signal())]
        pub fn send_service_signal(
            origin: OriginFor<T>,
            service_identifier: Vec<u8>,
            url: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::ServiceSignalSent(service_identifier, url, who));
            Ok(())
        }
    }
}
