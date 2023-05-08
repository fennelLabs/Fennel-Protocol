//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Certificate;
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
    send_certificate {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)
    verify {
        assert_eq!(Certificate::<T>::certificate_count(), 1);
    }

    send_more_certificates {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
    }: _(RawOrigin::Signed(caller), target)
    verify {
        assert_eq!(Certificate::<T>::certificate_count(), 3);
    }

    revoke_certificate {
        let target: T::AccountId = whitelisted_caller();
        let caller = get_origin::<T>("Anakin");
        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
    }: _(caller, target)
    verify {
        assert_eq!(Certificate::<T>::certificate_count(), 0);
    }
}

impl_benchmark_test_suite!(Certificate, crate::mock::new_test_ext(), crate::mock::Test);
