//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Signal;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

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

    #[benchmark]
    fn send_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = whitelisted_caller();

        for _ in 0..100_000 {
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                target.clone(),
                3,
            )?;
        }

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 0);

        assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 0);
        assert_last_event::<T>(Event::RatingSignalSent(target, 0, caller).into());

        Ok(())
    }

    #[benchmark]
    fn update_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = whitelisted_caller();

        // Generate a bunch of signals.
        for _ in 0..100_000 {
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                target.clone(),
                3,
            )?;
        }

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 1);

        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 1);
        assert_last_event::<T>(Event::RatingSignalUpdated(target, 1, caller).into());

        Ok(())
    }

    #[benchmark]
    fn revoke_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller = get_origin::<T>("Anakin");

        for _ in 0..100_000 {
            Signal::<T>::send_rating_signal(caller.clone().into(), target.clone(), 2)?;
        }

        #[extrinsic_call]
        _(caller, target.clone());

        let caller: T::AccountId = get_account::<T>("Anakin");
        assert!(!RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_last_event::<T>(Event::RatingSignalRevoked(target, caller).into());

        Ok(())
    }

    #[benchmark]
    fn send_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = whitelisted_caller();

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
        let caller: T::AccountId = whitelisted_caller();

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
