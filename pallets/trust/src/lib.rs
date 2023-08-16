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
    use sp_runtime::traits::One;

    use crate::weights::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        /// The maximum size of a trust parameter string
        type MaxTrustParameterSize: Get<u32>;
    }

    #[pallet::pallet]
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
    #[pallet::getter(fn trust_paramter_list)]
    /// An account and a parameter string to an integer value.
    pub type TrustParameterList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxTrustParameterSize>,
        u8,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Announce that a trust parameter has been set
        TrustParameterSet(T::AccountId),
        /// Announce that an account has issued trust to another account
        TrustIssued(T::AccountId, T::AccountId),
        /// Announce that an account has revoked trust from another account
        TrustRevoked(T::AccountId, T::AccountId),
        /// Announce that an account has requested trust from another account
        TrustRequest(T::AccountId, T::AccountId),
        /// Announce that an account has cancelled a trust request from another account
        TrustRequestRemoved(T::AccountId, T::AccountId),
        /// Announce that an account has removed trust from another account
        TrustIssuanceRemoved(T::AccountId, T::AccountId),
        /// Announce that an account has removed a trust revocation from another account
        TrustRevocationRemoved(T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The given action would cause the total number of trust actions to overflow
        StorageOverflow,
        /// The requested trust action already exists
        TrustExists,
        /// The requested trust action does not exist
        TrustNotFound,
        /// The requested trust request already exists
        TrustRequestExists,
        /// The requested trust request does not exist
        TrustRequestNotFound,
        /// The requested trust revocation already exists
        TrustRevocationExists,
        /// The requested trust revocation does not exist
        TrustRevocationNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Fully give `origin`'s trust to account `address`
        #[pallet::weight(<T as Config>::WeightInfo::issue_trust())]
        #[pallet::call_index(0)]
        pub fn issue_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if !<TrustIssuance<T>>::contains_key(&who, &address) {
                let total: u32 = <CurrentIssued<T>>::get();
                let new_total: u32 =
                    total.checked_add(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                <TrustIssuance<T>>::insert(&who, &address, total);
                <CurrentIssued<T>>::put(new_total);
                Self::deposit_event(Event::TrustIssued(who, address));
            } else {
                return Err(Error::<T>::TrustExists.into())
            }

            Ok(())
        }

        /// Remove issued trust from an account `address`, making their trust status 'Unknown'
        #[pallet::weight(<T as Config>::WeightInfo::remove_trust())]
        #[pallet::call_index(1)]
        pub fn remove_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if <TrustIssuance<T>>::contains_key(&who, &address) {
                let key = <CurrentIssued<T>>::get();
                let new_key: u32 =
                    key.checked_sub(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                <TrustIssuance<T>>::remove(&who, &address);
                <CurrentIssued<T>>::put(new_key);
                Self::deposit_event(Event::TrustIssuanceRemoved(address, who));
            } else {
                return Err(Error::<T>::TrustNotFound.into())
            }

            Ok(())
        }

        /// Place a request for `address` to issue explicit trust to the sender.
        #[pallet::weight(<T as Config>::WeightInfo::request_trust())]
        #[pallet::call_index(2)]
        pub fn request_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if !<TrustRequestList<T>>::contains_key(&who, &address) {
                let total: u32 = <CurrentRequests<T>>::get();
                let new_total: u32 =
                    total.checked_add(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                <CurrentRequests<T>>::put(new_total);
                <TrustRequestList<T>>::insert(&who, &address, total);
                Self::deposit_event(Event::TrustRequest(who, address));
            } else {
                return Err(Error::<T>::TrustRequestExists.into())
            }

            Ok(())
        }

        /// Rescind or cancel a trust request placed to `address`.
        #[pallet::weight(<T as Config>::WeightInfo::cancel_trust_request())]
        #[pallet::call_index(3)]
        pub fn cancel_trust_request(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if <TrustRequestList<T>>::contains_key(&who, &address) {
                let key = <CurrentRequests<T>>::get();
                let new_key: u32 =
                    key.checked_sub(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                <TrustRequestList<T>>::remove(&who, &address);
                <CurrentRequests<T>>::put(new_key);
                Self::deposit_event(Event::TrustRequestRemoved(address, who));
            } else {
                return Err(Error::<T>::TrustRequestNotFound.into())
            }

            Ok(())
        }

        /// Explcitly mark an account as untrusted
        #[pallet::weight(<T as Config>::WeightInfo::revoke_trust())]
        #[pallet::call_index(4)]
        pub fn revoke_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if !<TrustRevocation<T>>::contains_key(&who, &address) {
                let key: u32 = <CurrentRevoked<T>>::get();
                let new_key: u32 =
                    key.checked_add(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                <TrustRevocation<T>>::insert(&who, &address, key);
                <CurrentRevoked<T>>::put(new_key);
                Self::deposit_event(Event::TrustRevoked(address, who));
            } else {
                return Err(Error::<T>::TrustRevocationExists.into())
            }

            Ok(())
        }

        /// Return an untrusted `address` to an Unknown trust state
        #[pallet::weight(<T as Config>::WeightInfo::remove_revoked_trust())]
        #[pallet::call_index(5)]
        pub fn remove_revoked_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if <TrustRevocation<T>>::contains_key(&who, &address) {
                let key: u32 = <CurrentRevoked<T>>::get();
                let new_key: u32 =
                    key.checked_sub(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                <TrustRevocation<T>>::remove(&who, &address);
                <CurrentRevoked<T>>::put(new_key);
                Self::deposit_event(Event::TrustRevocationRemoved(address, who));
            } else {
                return Err(Error::<T>::TrustRevocationNotFound.into())
            }

            Ok(())
        }

        /// Defines coefficients that participants should use to weight rating functions.
        #[pallet::weight(<T as Config>::WeightInfo::set_trust_parameter())]
        #[pallet::call_index(6)]
        pub fn set_trust_parameter(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxTrustParameterSize>,
            value: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <TrustParameterList<T>>::insert(who.clone(), name.clone(), value);
            Self::deposit_event(Event::TrustParameterSet(who));

            Ok(())
        }
    }
}
