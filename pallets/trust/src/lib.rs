#![cfg_attr(not(feature = "std"), no_std)]

//!
//!
//! Three Levels of Trust:
//!
//! - I know you and I trust you (in Trusted List)
//! - I don't know anything about you so I don't trust you by default  (Not in revoked or trusted list)
//! - I know you and I definitely don't trust you (in the Revoked List)
//!
//! 

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch,
					traits::Get, storage::IterableStorageDoubleMap
};
use frame_system::ensure_signed;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TrustModule {
		CurrentIssued get(fn value1): u32;
		TrustIssuance get(fn key1): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => T::AccountId;
		CurrentRevoked get(fn value2): u32;
		TrustRevocation get(fn key2): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => T::AccountId;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
	{
		TrustIssued(AccountId, AccountId),
		TrustRevoked(AccountId, AccountId),
		TrustIssuanceRemoved(AccountId, AccountId),
		TrustRevocationRemoved(AccountId, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Fully give your trust to an account 
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn issue_trust(origin, address: T::AccountId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			// WARN: THIS ITERATION IS VERY INEFFICIENT
			// NOT SUITABLE FOR PRODUCTION
			let mut do_insert = true;
			let mut i = 0;
			for (_index, issued) in <TrustIssuance<T>>::iter_prefix(&who) {
				if issued == address { do_insert = false; }
				i += 1;
			}

			if do_insert {
				<TrustIssuance<T>>::insert(&who, i, &address);
				let total: u32 = CurrentIssued::get();
				CurrentIssued::put(total + 1);
				Self::deposit_event(RawEvent::TrustIssued(who, address));
			}
			
			Ok(())
		}

		/// Remove issued trust from an account, making their trust status 'Unknown'
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn remove_trust(origin, address: T::AccountId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let mut do_remove: Option<u32> = None;
			for (index, issued) in <TrustIssuance<T>>::iter_prefix(&who) {
				if issued == address { do_remove = Some(index); }
			}

			if let Some(index) = do_remove {
				<TrustIssuance<T>>::remove(&who, index);
				let key = CurrentIssued::get();
				CurrentIssued::put(key - 1);
				Self::deposit_event(RawEvent::TrustIssuanceRemoved(address, who));
			}

			Ok(())
		}

		/// Revoke trust from an account
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn revoke_trust(origin, address: T::AccountId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let mut do_insert = true;
			let mut i = 0;
			for (_index, revoked) in <TrustRevocation<T>>::iter_prefix(&who) {
				if revoked == address { do_insert = false }
				i += 1;
			}

			if do_insert {
				<TrustRevocation<T>>::insert(&who, i, &address);
				let key: u32 = CurrentRevoked::get();	
				CurrentRevoked::put(key + 1);
				Self::deposit_event(RawEvent::TrustRevoked(address, who));
			}
		
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn remove_revoked_trust(origin, address: T::AccountId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let mut do_remove: Option<u32> = None;

			for (index, revoked) in <TrustRevocation<T>>::iter_prefix(&who) {
				if revoked == address { do_remove = Some(index) }
			}
			
			if let Some(index) = do_remove {
				<TrustRevocation<T>>::remove(&who, index);	
				let key: u32 = CurrentRevoked::get();
				CurrentRevoked::put(key - 1);
				Self::deposit_event(RawEvent::TrustRevocationRemoved(address, who));
			}

			Ok(())
		}
	}
}
