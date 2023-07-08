//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Trust;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_trust_parameter() -> Result<(), BenchmarkError> {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), target, 0);

        Ok(())
    }

    #[benchmark]
    fn issue_trust() -> Result<(), BenchmarkError> {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentIssued::<T>::get(), 1);
        assert!(TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn issue_trust_repeatedly() -> Result<(), BenchmarkError> {
        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            let caller: T::AccountId = benchmark_account("caller", i, 0);
            Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        issue_trust(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentIssued::<T>::get(), 1001);
        assert!(TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn revoke_trust() -> Result<(), BenchmarkError> {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRevoked::<T>::get(), 1);
        assert!(TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn revoke_trust_from_heavy_storage() -> Result<(), BenchmarkError> {
        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            let caller: T::AccountId = benchmark_account("caller", i, 0);
            Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", 1000 + i, 0);
            let caller: T::AccountId = benchmark_account("caller", 1000 + i, 0);
            Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        revoke_trust(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRevoked::<T>::get(), 1001);
        assert!(TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn remove_trust() -> Result<(), BenchmarkError> {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentIssued::<T>::get(), 0);
        assert!(!TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn remove_trust_from_heavy_storage() -> Result<(), BenchmarkError> {
        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            let caller: T::AccountId = benchmark_account("caller", i, 0);
            Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

        #[extrinsic_call]
        remove_trust(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentIssued::<T>::get(), 1000);
        assert!(!TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn request_trust() -> Result<(), BenchmarkError> {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRequests::<T>::get(), 1);
        assert!(TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn request_trust_repeatedly() -> Result<(), BenchmarkError> {
        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            let caller: T::AccountId = benchmark_account("caller", i, 0);
            Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        request_trust(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRequests::<T>::get(), 1001);
        assert!(TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn remove_revoked_trust() -> Result<(), BenchmarkError> {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRevoked::<T>::get(), 0);
        assert!(!TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn remove_revoked_trust_heavy_storage() -> Result<(), BenchmarkError> {
        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            let caller: T::AccountId = benchmark_account("caller", i, 0);
            Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();

        Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

        #[extrinsic_call]
        remove_revoked_trust(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRevoked::<T>::get(), 1000);
        assert!(!TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn cancel_trust_request() -> Result<(), BenchmarkError> {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
        Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRequests::<T>::get(), 0);
        assert!(!TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    #[benchmark]
    fn cancel_trust_request_heavy_storage() -> Result<(), BenchmarkError> {
        for i in 0..1000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            let caller: T::AccountId = benchmark_account("caller", i, 0);
            Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
        }

        let target: T::AccountId = benchmark_account("target", 14, 0);
        let caller: T::AccountId = benchmark_account("caller", 14, 0);

        #[extrinsic_call]
        cancel_trust_request(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(CurrentRequests::<T>::get(), 999);
        assert!(!TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));

        Ok(())
    }

    impl_benchmark_test_suite!(Trust, crate::mock::new_test_ext(), crate::mock::Test);
}
