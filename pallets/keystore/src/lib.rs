#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	/// This module's main storage will consist of a StorageDoubleMap connecting addresses to the list of keys they've submitted and not revoked.
	#[pallet::getter(fn key)]
	pub type IssuedKeys<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, Vec<u8>, Vec<u8>>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Announce when an identity has broadcast a new key as an event.
		KeyIssued(Vec<u8>, T::AccountId),
		/// Announce when an identity has set a key as revoked.
		KeyRevoked(Vec<u8>, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// After a new key is created, call this extrinsic to announce it to the network.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn issue_key(origin: OriginFor<T>, fingerprint: Vec<u8>, location: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<IssuedKeys<T>>::insert(&who, &fingerprint, location);

			Self::deposit_event(Event::KeyIssued(fingerprint, who));
			Ok(())
		}

		/// If a key needs to be removed from circulation, this extrinsic will handle deleting it.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn revoke_key(origin: OriginFor<T>, key_index: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<IssuedKeys<T>>::remove(&who, &key_index);

			Self::deposit_event(Event::KeyRevoked(key_index, who));
			Ok(())
		}
	}
}
