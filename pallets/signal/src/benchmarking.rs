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

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn send_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 0);

        assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 0);

        Ok(())
    }

    #[benchmark]
    fn update_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 1);

        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 1);

        Ok(())
    }

    #[benchmark]
    fn revoke_rating_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller = get_origin::<T>("Anakin");
        Signal::<T>::send_rating_signal(caller.clone().into(), target.clone(), 0)?;

        #[extrinsic_call]
        _(caller, target.clone());

        let caller: T::AccountId = get_account::<T>("Anakin");
        assert!(!RatingSignalList::<T>::contains_key(caller, target.clone()));

        Ok(())
    }

    #[benchmark]
    fn send_signal() -> Result<(), BenchmarkError> {
        let target =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
                .unwrap();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), target.into());

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

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), service, url);

        Ok(())
    }

    impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);
}
