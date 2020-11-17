#![cfg_attr(not(feature = "std"), no_std)]

use crate::dispatch::Vec;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		IssuedKeys get(fn key): map hasher(blake2_128_concat) Vec<u8> => Vec<u8>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
	{
		KeyIssued(Vec<u8>, AccountId),
		KeyRevoked(Vec<u8>, AccountId),
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

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn issue_key(origin, fingerprint: Vec<u8>, hash: Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			IssuedKeys::insert(&fingerprint, hash);

			Self::deposit_event(RawEvent::KeyIssued(fingerprint, who));
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn revoke_key(origin, key_index: Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			IssuedKeys::remove(&key_index);

			Self::deposit_event(RawEvent::KeyRevoked(key_index, who));
			Ok(())
		}
	}
}
