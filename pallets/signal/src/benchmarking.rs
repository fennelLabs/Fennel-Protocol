//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Signal;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::{sp_runtime::traits::Bounded, traits::Currency, BoundedVec};
use frame_system::RawOrigin;
use scale_info::prelude::format;
use scale_info::prelude::vec;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
    use super::*;
    type DepositBalanceOf<T> = <<T as pallet::Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[benchmark]
    fn set_signal_parameter() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = get_account::<T>("//Alice");

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 0);

        assert!(SignalParameterList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(SignalParameterList::<T>::get(caller.clone(), target.clone()), 0);
        assert_last_event::<T>(Event::SignalParameterSet(caller).into());

        Ok(())
    }

    #[benchmark]
    fn send_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = get_account::<T>("//Alice");

        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TEST{}", i).as_bytes().to_vec(),
            )
            .unwrap();
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                loop_target,
                3,
            )?;
        }

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 0);

        assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 0);
        assert_last_event::<T>(Event::RatingSignalSent(caller).into());

        Ok(())
    }

    #[benchmark]
    fn update_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();

        let caller: T::AccountId = get_account::<T>("//Alice");
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

        // Generate a bunch of signals.
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TEST{}", i).as_bytes().to_vec(),
            )
            .unwrap();
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                loop_target,
                3,
            )?;
        }

        Signal::<T>::send_rating_signal(
            RawOrigin::Signed(caller.clone()).into(),
            target.clone(),
            3,
        )?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 1);

        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 1);
        assert_last_event::<T>(Event::RatingSignalUpdated(caller).into());

        Ok(())
    }

    #[benchmark]
    fn revoke_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();

        let caller = get_origin::<T>("Anakin");
        let caller_account = get_account::<T>("Anakin");
        T::Currency::make_free_balance_be(&caller_account, DepositBalanceOf::<T>::max_value());

        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TEST{}", i).as_bytes().to_vec(),
            )
            .unwrap();
            Signal::<T>::send_rating_signal(caller.clone().into(), loop_target, 2)?;
        }

        Signal::<T>::send_rating_signal(caller.clone().into(), target.clone(), 2)?;

        #[extrinsic_call]
        _(caller, target.clone());

        let caller: T::AccountId = get_account::<T>("Anakin");
        assert!(!RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_last_event::<T>(Event::RatingSignalRevoked(caller).into());

        Ok(())
    }

    #[benchmark]
    fn send_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = get_account::<T>("//Alice");

        for _ in 0..100_000 {
            Signal::<T>::send_signal(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone().into());

        assert_last_event::<T>(Event::SignalSent(target, caller).into());

        Ok(())
    }

    #[benchmark]
    fn send_service_signal() -> Result<(), BenchmarkError> {
        let service =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let url =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = get_account::<T>("//Alice");

        for _ in 0..100_000 {
            Signal::<T>::send_service_signal(
                RawOrigin::Signed(caller.clone()).into(),
                service.clone(),
                url.clone(),
            )?;
        }

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), service.clone(), url.clone());

        assert_last_event::<T>(Event::ServiceSignalSent(service, url, caller).into());

        Ok(())
    }

    impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);
}
