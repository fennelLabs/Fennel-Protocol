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
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::ensure_signed;
    use frame_system::pallet_prelude::*;

    const LOCK_ID: LockIdentifier = *b"fennelc ";

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        Locked(<T as frame_system::Config>::AccountId, BalanceOf<T>),
        ExtendedLock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
        Unlocked(<T as frame_system::Config>::AccountId),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn lock_capital(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::set_lock(LOCK_ID, &user, amount, WithdrawReasons::all());

            Self::deposit_event(Event::Locked(user, amount));
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn extend_lock(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::extend_lock(LOCK_ID, &user, amount, WithdrawReasons::all());

            Self::deposit_event(Event::ExtendedLock(user, amount));
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn unlock_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::remove_lock(LOCK_ID, &user);

            Self::deposit_event(Event::Unlocked(user));
            Ok(().into())
        }
    }
}
