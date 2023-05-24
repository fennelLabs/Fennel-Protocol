//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Certificate;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
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
        let s in 0 .. 100000;
        let target = get_account::<T>("James");
        let caller = get_origin::<T>("Spock");
    }: _(caller, target)
    verify {
        let caller_account_id: T::AccountId = get_account::<T>("Spock");
        let target_account_id: T::AccountId = get_account::<T>("James");
        assert!(CertificateList::<T>::contains_key(caller_account_id, target_account_id));
    }

    revoke_certificate {
        let s in 0 .. 100000;
        let target = get_account::<T>("Montgomery");
        let caller = get_origin::<T>("Leonard");
        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
    }: _(caller, target)
    verify {
        let caller_account_id: T::AccountId = get_account::<T>("Leonard");
        let target_account_id: T::AccountId = get_account::<T>("Montgomery");
        assert!(!CertificateList::<T>::contains_key(caller_account_id, target_account_id));
    }
}

impl_benchmark_test_suite!(Certificate, crate::mock::new_test_ext(), crate::mock::Test);
