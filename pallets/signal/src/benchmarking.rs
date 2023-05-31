//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Signal;
use frame_benchmarking::{
    account as benchmark_account, benchmarks, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_system::RawOrigin;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

benchmarks! {
    set_signal_parameter {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target, 0)

    send_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), target.clone(), 0)
    verify {
        assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 0);
    }

    send_whiteflag_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target, 0)

    update_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), target.clone(), 1)
    verify {
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 1);
    }

    update_whiteflag_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target, 0)

    revoke_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller = get_origin::<T>("Anakin");
        Signal::<T>::send_rating_signal(caller.clone().into(), target.clone(), 0)?;
    }: _(caller, target.clone())
    verify {
        let caller: T::AccountId = get_account::<T>("Anakin");
        assert!(!RatingSignalList::<T>::contains_key(caller, target.clone()));
    }

    send_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    revoke_whiteflag_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller = get_origin::<T>("Anakin");
        Signal::<T>::send_whiteflag_rating_signal(caller.clone().into(), target.clone(), 0)?;
    }: _(caller, target)

    send_service_signal {
        let service = "TEST".as_bytes().to_vec();
        let url = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), service, url)
}

impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);
