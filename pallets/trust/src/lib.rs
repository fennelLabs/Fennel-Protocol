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

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 {
        0
    }
    #[pallet::storage]
    #[pallet::getter(fn get_current_trust_count)]
    /// The total number of trust actions currently active
    pub type CurrentIssued<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;
    #[pallet::storage]
    #[pallet::getter(fn get_trust_issuance)]
    /// A Map of lists of all addresses that each address has issued trust for
    pub type TrustIssuance<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;
    #[pallet::storage]
    #[pallet::getter(fn get_current_non_trust_count)]
    /// The current number of _non_trust actions currently active
    pub type CurrentRevoked<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;
    #[pallet::storage]
    #[pallet::getter(fn get_non_trust_issuance)]
    /// A Map of lists of all addresses that each address has revoked trust for
    pub type TrustRevocation<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;
    #[pallet::storage]
    #[pallet::getter(fn get_current_trust_requests)]
    pub type CurrentRequests<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;
    #[pallet::storage]
    #[pallet::getter(fn get_trust_request)]
    /// A map listing all requests for trust from one account to another.
    pub type TrustRequestList<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn trust_paramter_list)]
    /// An account and a parameter string to an integer value.
    pub type TrustParameterList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        Vec<u8>,
        u8,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TrustParameterSet(Vec<u8>, u8, T::AccountId),
        TrustIssued(T::AccountId, T::AccountId),
        TrustRevoked(T::AccountId, T::AccountId),
        TrustRequest(T::AccountId, T::AccountId),
        TrustRequestRemoved(T::AccountId, T::AccountId),
        TrustIssuanceRemoved(T::AccountId, T::AccountId),
        TrustRevocationRemoved(T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Defines coefficients that participants should use to weight rating functions.
        #[pallet::weight(<T as Config>::WeightInfo::set_trust_parameter())]
        pub fn set_trust_parameter(
            origin: OriginFor<T>,
            name: Vec<u8>,
            value: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <TrustParameterList<T>>::insert(who.clone(), name.clone(), value);
            Self::deposit_event(Event::TrustParameterSet(name, value, who));

            Ok(())
        }

        /// Fully give `origin`'s trust to account `address`
        #[pallet::weight(<T as Config>::WeightInfo::issue_trust())]
        pub fn issue_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if !<TrustIssuance<T>>::contains_key(&who, &address) {
                let total: u32 = <CurrentIssued<T>>::get();
                let new_total: u32 = total.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                <TrustIssuance<T>>::insert(&who, &address, total);
                <CurrentIssued<T>>::put(new_total);
                Self::deposit_event(Event::TrustIssued(who, address));
            }

            Ok(())
        }

        /// Remove issued trust from an account `address`, making their trust status 'Unknown'
        #[pallet::weight(<T as Config>::WeightInfo::remove_trust())]
        pub fn remove_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if <TrustIssuance<T>>::contains_key(&who, &address) {
                let key = <CurrentIssued<T>>::get();
                let new_key: u32 = key.checked_sub(1).ok_or(Error::<T>::StorageOverflow)?;
                <TrustIssuance<T>>::remove(&who, &address);
                <CurrentIssued<T>>::put(new_key);
                Self::deposit_event(Event::TrustIssuanceRemoved(address, who));
            }

            Ok(())
        }

        /// Place a request for `address` to issue explicit trust to the sender.
        #[pallet::weight(<T as Config>::WeightInfo::request_trust())]
        pub fn request_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if !<TrustRequestList<T>>::contains_key(&who, &address) {
                let total: u32 = <CurrentRequests<T>>::get();
                let new_total: u32 = total.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                <CurrentRequests<T>>::put(new_total);
                <TrustRequestList<T>>::insert(&who, &address, total);
                Self::deposit_event(Event::TrustRequest(who, address));
            }

            Ok(())
        }

        /// Rescind or cancel a trust request placed to `address`.
        #[pallet::weight(<T as Config>::WeightInfo::cancel_trust_request())]
        pub fn cancel_trust_request(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if <TrustRequestList<T>>::contains_key(&who, &address) {
                let key = <CurrentRequests<T>>::get();
                let new_key: u32 = key.checked_sub(1).ok_or(Error::<T>::StorageOverflow)?;
                <TrustRequestList<T>>::remove(&who, &address);
                <CurrentRequests<T>>::put(new_key);
                Self::deposit_event(Event::TrustRequestRemoved(address, who));
            }

            Ok(())
        }

        /// Explcitly mark an account as untrusted
        #[pallet::weight(<T as Config>::WeightInfo::revoke_trust())]
        pub fn revoke_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if !<TrustRevocation<T>>::contains_key(&who, &address) {
                let key: u32 = <CurrentRevoked<T>>::get();
                let new_key: u32 = key.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                <TrustRevocation<T>>::insert(&who, &address, key);
                <CurrentRevoked<T>>::put(new_key);
                Self::deposit_event(Event::TrustRevoked(address, who));
            }

            Ok(())
        }

        /// Return an untrusted `address` to an Unknown trust state
        #[pallet::weight(<T as Config>::WeightInfo::remove_revoked_trust())]
        pub fn remove_revoked_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if <TrustRevocation<T>>::contains_key(&who, &address) {
                let key: u32 = <CurrentRevoked<T>>::get();
                let new_key: u32 = key.checked_sub(1).ok_or(Error::<T>::StorageOverflow)?;
                <TrustRevocation<T>>::remove(&who, &address);
                <CurrentRevoked<T>>::put(new_key);
                Self::deposit_event(Event::TrustRevocationRemoved(address, who));
            }

            Ok(())
        }
    }
}
