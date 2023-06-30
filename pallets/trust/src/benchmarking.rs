//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Trust;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

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

    impl_benchmark_test_suite!(Trust, crate::mock::new_test_ext(), crate::mock::Test);
}
