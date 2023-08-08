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
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        /// The maximum size of a signal.
        type MaxSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
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
        /// Represents a signal sent by an identity.
        SignalSent(BoundedVec<u8, T::MaxSize>, T::AccountId),
        /// Represents a signal sent by an identity for a particular application or service.
        ServiceSignalSent(BoundedVec<u8, T::MaxSize>, BoundedVec<u8, T::MaxSize>, T::AccountId),
        /// Indicates that an identity issued a new rating signal.
        RatingSignalSent(T::AccountId),
        /// Indicates that an identity updated a rating signal.
        RatingSignalUpdated(T::AccountId),
        /// Indicates that an identity revoked a rating signal.
        RatingSignalRevoked(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Requested rating signal already exists.
        RatingSignalAlreadyExists,
        /// Requested rating signal does not exist.
        RatingSignalDoesNotExist,
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

            ensure!(
                !<RatingSignalList<T>>::contains_key(who.clone(), target.clone()),
                Error::<T>::RatingSignalAlreadyExists
            );

            <RatingSignalList<T>>::insert(who.clone(), target.clone(), rating);
            Self::deposit_event(Event::RatingSignalSent(who));

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

            ensure!(
                <RatingSignalList<T>>::contains_key(who.clone(), target.clone()),
                Error::<T>::RatingSignalDoesNotExist
            );

            <RatingSignalList<T>>::insert(who.clone(), target.clone(), new_rating);
            Self::deposit_event(Event::RatingSignalUpdated(who));

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

            ensure!(
                <RatingSignalList<T>>::contains_key(who.clone(), target.clone()),
                Error::<T>::RatingSignalDoesNotExist
            );

            <RatingSignalList<T>>::remove(who.clone(), target.clone());
            Self::deposit_event(Event::RatingSignalRevoked(who));

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
