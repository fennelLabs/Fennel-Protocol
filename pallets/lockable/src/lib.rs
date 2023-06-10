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
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::{ensure_signed, pallet_prelude::*};

    use crate::weights::WeightInfo;

    const LOCK_ID: LockIdentifier = *b"fennelc ";

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
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

    #[pallet::error]
    #[derive(PartialEq, Eq)]
    pub enum Error<T> {
        InsufficientBalance,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as Config>::WeightInfo::lock_capital())]
        #[pallet::call_index(0)]
        pub fn lock_capital(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::set_lock(LOCK_ID, &user, amount, WithdrawReasons::all());

            Self::deposit_event(Event::Locked(user, amount));
            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::extend_lock())]
        #[pallet::call_index(1)]
        pub fn extend_lock(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::extend_lock(LOCK_ID, &user, amount, WithdrawReasons::all());

            Self::deposit_event(Event::ExtendedLock(user, amount));
            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::unlock_all())]
        #[pallet::call_index(2)]
        pub fn unlock_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::remove_lock(LOCK_ID, &user);

            Self::deposit_event(Event::Unlocked(user));
            Ok(().into())
        }
    }
}
