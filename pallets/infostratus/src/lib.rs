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

const ASSIGNMENT_EXISTS: bool = true;
const ASSIGNMENT_DOES_NOT_EXIST: bool = false;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use crate::{weights::WeightInfo, ASSIGNMENT_EXISTS, ASSIGNMENT_DOES_NOT_EXIST};

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        type Currency: LockableCurrency<
            Self::AccountId,
            Moment = frame_system::pallet_prelude::BlockNumberFor<Self>,
        >;
        /// The maximum length of a submission resource location.
        type MaxSize: Get<u32>;
        /// The identifier for the lock used to store deposits.
        type LockId: Get<LockIdentifier>;
        /// The price of a lock.
        type LockPrice: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn assignments_list)]
    /// Maps accounts to the array of identities it owns.
    pub type AssignmentsList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        bool,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn submissions_list)]
    /// Maps accounts to the submissions they've sent.
    pub type SubmissionsList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SubmissionSent(T::AccountId, BoundedVec<u8, T::MaxSize>),
        SubmissionAssigned(BoundedVec<u8, T::MaxSize>, T::AccountId),
        InfostratusLock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
        InfostratusUnlock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        SubmissionDoesNotExist,
        SubmissionExists,
        SubmissionAlreadyAssigned,
        InsufficientBalance,
        CannotAssignOwnSubmission,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event that a submission has been sent.
        /// This means that the origin wants a piece of online information verified by the community.
        #[pallet::weight(T::WeightInfo::create_submission_entry())]
        #[pallet::call_index(0)]
        pub fn create_submission_entry(origin: OriginFor<T>, resource_location: BoundedVec<u8, T::MaxSize>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into())
            }

            ensure!(
                !SubmissionsList::<T>::contains_key(&who, &resource_location),
                Error::<T>::SubmissionExists
            );
            // Insert a placeholder value into storage - if the pair (who, recipient) exists, we
            // know there's a certificate present for the pair, regardless of value.
            T::Currency::set_lock(T::LockId::get(), &who, 10u32.into(), WithdrawReasons::all());

            Self::deposit_event(Event::InfostratusLock(
                who.clone(),
                T::Currency::free_balance(&who),
            ));

            <SubmissionsList<T>>::insert(&who, resource_location.clone(), ASSIGNMENT_DOES_NOT_EXIST);

            Self::deposit_event(Event::SubmissionSent(who, resource_location));

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::request_submission_assignment())]
        #[pallet::call_index(1)]
        /// Creates an on-chain event that a submission has been assigned for the origin to verify.
        pub fn request_submission_assignment(origin: OriginFor<T>, poster: T::AccountId, resource_location: BoundedVec<u8, T::MaxSize>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into())
            }

            ensure!(
                &who != &poster,
                Error::<T>::CannotAssignOwnSubmission
            );

            ensure!(
                SubmissionsList::<T>::contains_key(&poster, &resource_location),
                Error::<T>::SubmissionDoesNotExist
            );

            ensure!(
                !SubmissionsList::<T>::get(&poster, &resource_location),
                Error::<T>::SubmissionAlreadyAssigned
            );

            T::Currency::set_lock(T::LockId::get(), &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::InfostratusUnlock(
                who.clone(),
                T::Currency::free_balance(&who),
            ));

            <AssignmentsList<T>>::insert(&who, resource_location.clone(), ASSIGNMENT_EXISTS);
            <SubmissionsList<T>>::try_mutate(&poster, resource_location.clone(), |value| -> DispatchResult {
                *value = ASSIGNMENT_EXISTS;
                Ok(())
            })?;

            Self::deposit_event(Event::SubmissionAssigned(resource_location, who));

            Ok(())
        }
    }
}
