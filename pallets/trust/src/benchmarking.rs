//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Trust;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    set_trust_parameter {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target, 0)

    issue_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), target.clone())
    verify {
        assert_eq!(CurrentIssued::<T>::get(), 1);
        assert!(TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));
    }

    revoke_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), target.clone())
    verify {
        assert_eq!(CurrentRevoked::<T>::get(), 1);
        assert!(TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));
    }

    remove_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
    }: _(RawOrigin::Signed(caller.clone()), target.clone())
    verify {
        assert_eq!(CurrentIssued::<T>::get(), 0);
        assert!(!TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));
    }

    remove_revoked_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
    }: _(RawOrigin::Signed(caller.clone()), target.clone())
    verify {
        assert_eq!(CurrentRevoked::<T>::get(), 0);
        assert!(!TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));
    }

    request_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), target.clone())
    verify {
        assert_eq!(CurrentRequests::<T>::get(), 1);
        assert!(TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));
    }

    cancel_trust_request {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
    }: _(RawOrigin::Signed(caller.clone()), target.clone())
    verify {
        assert_eq!(CurrentRequests::<T>::get(), 0);
        assert!(!TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));
    }
}

impl_benchmark_test_suite!(Trust, crate::mock::new_test_ext(), crate::mock::Test);
