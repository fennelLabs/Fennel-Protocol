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
        // Can we add a one line doc comment for each config? Please refer to my suggestions made in certificate and identity pallet .
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type MaxSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    // Why do we need it as Unbounded?
    #[pallet::unbounded]
    #[pallet::getter(fn rating_signal_list)]
    /// Maps identity numbers to a signal transaction hash and a rating number.
    pub type RatingSignalList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        u8,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Can we add a one line doc comment for each event?
        // Please pass only relevant information in event.
        // Substrate events are  "expensive". Events are created the same as any other storage item and consume the same amount of storage, based on the type of the inner data being emitted.
        SignalSent(BoundedVec<u8, T::MaxSize>, T::AccountId),
        RatingSignalSent(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        RatingSignalUpdated(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        RatingSignalRevoked(BoundedVec<u8, T::MaxSize>, T::AccountId),
        ServiceSignalSent(BoundedVec<u8, T::MaxSize>, BoundedVec<u8, T::MaxSize>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        // NoneValue,
        // StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a transaction hash as a pointer and a u8 as a rating
        /// number.
        #[pallet::weight(<T as Config>::WeightInfo::send_rating_signal())]
        #[pallet::call_index(0)]
        pub fn send_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
            rating: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // What if data already exist?
            <RatingSignalList<T>>::insert(who.clone(), target.clone(), rating);
            Self::deposit_event(Event::RatingSignalSent(target, rating, who));

            Ok(())
        }

        /// Updates an existing rating signal.
        #[pallet::weight(<T as Config>::WeightInfo::update_rating_signal())]
        #[pallet::call_index(1)]
        pub fn update_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
            new_rating: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // I don't see any difference between this extrinsic and `send_rating_signal`.
            // Can we add some validation check? What if data does not exist?

            <RatingSignalList<T>>::insert(who.clone(), target.clone(), new_rating);
            Self::deposit_event(Event::RatingSignalUpdated(target, new_rating, who));

            Ok(())
        }

        /// Puts out a signal cancelling a previous rating.
        #[pallet::weight(<T as Config>::WeightInfo::revoke_rating_signal())]
        #[pallet::call_index(2)]
        pub fn revoke_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Can we add some validation check? What if data does not exist?
            <RatingSignalList<T>>::remove(who.clone(), target.clone());
            Self::deposit_event(Event::RatingSignalRevoked(target, who));

            Ok(())
        }

        /// Creates an on-chain event with a signal payload defined as part of the transaction
        /// without relying on storage.
        #[pallet::weight(<T as Config>::WeightInfo::send_signal())]
        #[pallet::call_index(3)]
        pub fn send_signal(
            origin: OriginFor<T>,
            signal: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::SignalSent(signal, who));
            Ok(())
        }

        /// Sends a hexadecimal signal tagged for a particular application or service using Fennel
        /// Protocol.
        #[pallet::weight(<T as Config>::WeightInfo::send_service_signal())]
        #[pallet::call_index(4)]
        pub fn send_service_signal(
            origin: OriginFor<T>,
            service_identifier: BoundedVec<u8, T::MaxSize>,
            url: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::ServiceSignalSent(service_identifier, url, who));
            Ok(())
        }
    }
}
