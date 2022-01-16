#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn DefaultCurrent<T: Config>() -> u32 { 0 }
	#[pallet::storage]
	#[pallet::getter(fn get_current_trust_count)]
	/// The total number of trust actions currently active
	pub type CurrentIssued<T: Config> = StorageValue<Value = u32, QueryKind= ValueQuery, OnEmpty = DefaultCurrent<T>>;
	#[pallet::storage]
	#[pallet::getter(fn get_trust_issuance)]
	/// A Map of lists of all addresses that each address has issued trust for
	pub type TrustIssuance<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;
	#[pallet::storage]
	#[pallet::getter(fn get_current_non_trust_count)]
	/// The current number of _non_trust actions currently active
	pub type CurrentRevoked<T: Config> = StorageValue<Value = u32, QueryKind= ValueQuery, OnEmpty = DefaultCurrent<T>>;
	#[pallet::storage]
	#[pallet::getter(fn get_non_trust_issuance)]
	/// A Map of lists of all addresses that each address has issued trust for
	pub type TrustRevocation<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;
	#[pallet::storage]
	#[pallet::getter(fn get_current_trust_requests)]
	pub type CurrentRequests<T: Config> = StorageValue<Value = u32, QueryKind= ValueQuery, OnEmpty = DefaultCurrent<T>>;
	#[pallet::storage]
	#[pallet::getter(fn get_trust_request)]
	/// A map listing all requests for trust from one account to another.
	pub type TrustRequestList<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
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
		/// Fully give `origin`'s trust to account `address`
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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
